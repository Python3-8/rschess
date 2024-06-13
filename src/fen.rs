use super::{helpers, Color, InvalidFenError, Piece, PieceType, Position};
use std::fmt;

/// Represents FEN (Forsyth-Edwards Notation).
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Fen {
    pub(crate) position: Position,
    pub(crate) halfmove_clock: usize,
    pub(crate) fullmove_number: usize,
}

impl Fen {
    /// Returns the position represented by the `Fen` object.
    pub fn position(&self) -> &Position {
        &self.position
    }

    /// Returns the halfmove clock.
    pub fn halfmove_clock(&self) -> usize {
        self.halfmove_clock
    }

    /// Returns the fullmove number.
    pub fn fullmove_number(&self) -> usize {
        self.fullmove_number
    }
}

impl TryFrom<&str> for Fen {
    type Error = InvalidFenError;

    /// Attempts to construct a `Fen` object from a string slice, returning an error if it is invalid.
    /// **Shredder-FEN is NOT supported**.
    fn try_from(fen: &str) -> Result<Self, Self::Error> {
        let mut content = [None; 64];
        let fields: Vec<_> = fen.split(' ').collect();
        let nfields = fields.len();
        if nfields != 6 {
            return Err(InvalidFenError::SixFields);
        }
        let ranks: Vec<_> = fields[0].split('/').collect();
        let nranks = ranks.len();
        if nranks != 8 {
            return Err(InvalidFenError::BoardData(format!("expected eight ranks of pieces separated by forward-slashes, got {nranks}")));
        }
        let mut wk_seen = false;
        let mut wk_pos = 0;
        let mut bk_seen = false;
        let mut bk_pos = 0;
        let mut ptr: usize = 63;
        let mut rankn = 8;
        for rank in ranks {
            let mut rank_filled = 0;
            for piece_char in rank.chars().rev() {
                if rank_filled == 8 {
                    return Err(InvalidFenError::BoardData(format!("rank {rankn} cannot have pieces beyond the h file (8 squares already occupied)")));
                }
                if piece_char.is_ascii_digit() {
                    let empty_space = piece_char.to_digit(10).unwrap() as usize;
                    if !(1..=8).contains(&empty_space) {
                        return Err(InvalidFenError::BoardData(format!(
                            "{empty_space} is not a valid character for board data, digits must be in the range 1..=8"
                        )));
                    }
                    if empty_space > 8 - rank_filled {
                        return Err(InvalidFenError::BoardData(format!(
                            "rank {rankn} only has 8 squares, {rank_filled} of which is/are occupied. {empty_space} more squares of empty space cannot be accomodated"
                        )));
                    }
                    rank_filled += empty_space;
                    ptr = ptr.saturating_sub(empty_space);
                } else {
                    content[ptr] = match piece_char.try_into() {
                        Ok(piece) => {
                            match piece {
                                Piece(PieceType::K, Color::White) => {
                                    if wk_seen {
                                        return Err(InvalidFenError::BoardData("white cannot have more than one king".to_owned()));
                                    }
                                    wk_seen = true;
                                    wk_pos = ptr;
                                }
                                Piece(PieceType::K, Color::Black) => {
                                    if bk_seen {
                                        return Err(InvalidFenError::BoardData("black cannot have more than one king".to_owned()));
                                    }
                                    bk_seen = true;
                                    bk_pos = ptr;
                                }
                                Piece(PieceType::P, _) => {
                                    if !(8..56).contains(&ptr) {
                                        return Err(InvalidFenError::BoardData("there cannot be pawns on the 1st and 8th ranks".to_owned()));
                                    }
                                }
                                _ => (),
                            }
                            Some(piece)
                        }
                        Err(e) => return Err(InvalidFenError::BoardData(e.to_string())),
                    };
                    rank_filled += 1;
                    ptr = ptr.saturating_sub(1);
                }
            }
            if rank_filled != 8 {
                return Err(InvalidFenError::BoardData(format!("rank {rankn} does not have data occupying all 8 squares")));
            }
            rankn -= 1;
        }
        if !(wk_seen && bk_seen) {
            return Err(InvalidFenError::BoardData("a valid chess position must have one white king and one black king".to_owned()));
        }
        let turn = fields[1];
        let side = match Color::try_from(turn) {
            Ok(c) => c,
            _ => return Err(InvalidFenError::ActiveColor),
        };
        if helpers::king_capture_pseudolegal(&content, side) {
            return Err(InvalidFenError::BoardData("when one side is in check, it cannot be the other side's turn to move".to_owned()));
        }
        let castling = fields[2];
        let len_castling = castling.len();
        if !((1..=4).contains(&len_castling)) {
            return Err(InvalidFenError::CastlingRights("expected castling rights to be 1 to 4 characters long".to_owned()));
        }
        let mut castling_rights_old = [false; 4];
        if castling != "-" {
            for ch in castling.chars() {
                match ch {
                    'K' => {
                        if wk_pos > 6 {
                            return Err(InvalidFenError::CastlingRights("white king must be from a1 to g1 to have kingside castling rights".to_owned()));
                        }
                        if castling_rights_old[0] {
                            return Err(InvalidFenError::CastlingRights("found more than one occurrence of 'K'".to_owned()));
                        }
                        castling_rights_old[0] = true;
                    }
                    'Q' => {
                        if !(1..=7).contains(&wk_pos) {
                            return Err(InvalidFenError::CastlingRights("white king must be from b1 to h1 to have queenside castling rights".to_owned()));
                        }
                        if castling_rights_old[1] {
                            return Err(InvalidFenError::CastlingRights("found more than one occurrence of 'Q'".to_owned()));
                        }
                        castling_rights_old[1] = true;
                    }
                    'k' => {
                        if !(56..=62).contains(&bk_pos) {
                            return Err(InvalidFenError::CastlingRights("black king must be from a8 to g8 to have kingside castling rights".to_owned()));
                        }
                        if castling_rights_old[2] {
                            return Err(InvalidFenError::CastlingRights("found more than one occurrence of 'k'".to_owned()));
                        }
                        castling_rights_old[2] = true;
                    }
                    'q' => {
                        if !(57..=63).contains(&bk_pos) {
                            return Err(InvalidFenError::CastlingRights("black king must be from b8 to h8 to have queenside castling rights".to_owned()));
                        }
                        if castling_rights_old[3] {
                            return Err(InvalidFenError::CastlingRights("found more than one occurrence of 'q'".to_owned()));
                        }
                        castling_rights_old[3] = true;
                    }
                    _ => return Err(InvalidFenError::CastlingRights("expected '-' or a subset of 'KQkq'".to_owned())),
                }
            }
        }
        let count_rooks = |rng, color| helpers::count_piece(rng, Piece(PieceType::R, color), &content);
        if castling_rights_old[0] && count_rooks(wk_pos + 1..8, Color::White) != 1 {
            return Err(InvalidFenError::CastlingRights("white must have exactly one king's rook to have kingside castling rights".to_owned()));
        }
        if castling_rights_old[1] && count_rooks(0..wk_pos, Color::White) != 1 {
            return Err(InvalidFenError::CastlingRights("white must have exactly one queen's rook to have queenside castling rights".to_owned()));
        }
        if castling_rights_old[2] && count_rooks(bk_pos + 1..64, Color::Black) != 1 {
            return Err(InvalidFenError::CastlingRights("black must have exactly one king's rook to have kingside castling rights".to_owned()));
        }
        if castling_rights_old[3] && count_rooks(56..bk_pos, Color::Black) != 1 {
            return Err(InvalidFenError::CastlingRights("black must have exactly one queen's rook to have queenside castling rights".to_owned()));
        }
        let find_rook = |rng, color| helpers::find_pieces(Piece(PieceType::R, color), rng, &content)[0];
        let mut castling_rights = [None; 4];
        if castling_rights_old[0] {
            castling_rights[0] = Some(find_rook(wk_pos + 1..8, Color::White));
        }
        if castling_rights_old[1] {
            castling_rights[1] = Some(find_rook(0..wk_pos, Color::White));
        }
        if castling_rights_old[2] {
            castling_rights[2] = Some(find_rook(bk_pos + 1..64, Color::Black));
        }
        if castling_rights_old[3] {
            castling_rights[3] = Some(find_rook(56..bk_pos, Color::Black));
        }
        let ep = fields[3];
        let len_ep = ep.len();
        if !((1..=2).contains(&len_ep)) {
            return Err(InvalidFenError::EnPassantTargetSquare);
        }
        let mut ep_target = None;
        if ep != "-" {
            let err = Err(InvalidFenError::EnPassantTargetSquare);
            if len_ep != 2 {
                return err;
            }
            let file = ep.chars().next().unwrap();
            let rank = ep.chars().nth(1).unwrap();
            if !(('a'..='h').contains(&file) && ['3', '6'].contains(&rank)) {
                return err;
            }
            ep_target = Some(helpers::sq_to_idx(file, rank));
        }
        let position = Position {
            content,
            side,
            castling_rights,
            ep_target,
        };
        let halfmoves = fields[4];
        let halfmove_clock: usize = halfmoves.parse().map_err(|_| InvalidFenError::HalfmoveClock)?;
        if halfmove_clock > 150 {
            return Err(InvalidFenError::HalfmoveClock);
        }
        let fullmoves = fields[5];
        let fullmove_number: usize = fullmoves.parse().map_err(|_| InvalidFenError::FullmoveNumber)?;
        if fullmove_number < 1 {
            return Err(InvalidFenError::FullmoveNumber);
        }
        Ok(Self {
            position,
            halfmove_clock,
            fullmove_number,
        })
    }
}

impl fmt::Display for Fen {
    /// Returns an FEN string representing this object.
    /// If standard FEN is inadequate for representing castling rights, a mixture of standard FEN and Shredder-FEN will be generated.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", [self.position.to_fen(), self.halfmove_clock.to_string(), self.fullmove_number.to_string()].join(" "))
    }
}

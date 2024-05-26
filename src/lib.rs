//! # rschess
//! rschess is yet another chess library for Rust, with the aim to be as feature-rich as possible. It is still IN DEVELOPMENT, and NOT FIT FOR USE.

/// The structure for a rschess chessboard
#[derive(Debug)]
pub struct Board {
    /// The board content; each square is represented by a number 0..64 where a1 is 0 and h8 is 63
    content: [Occupant; 64],
    /// The side to move; white is `true` and black is `false`
    side_to_move: bool,
    /// The castling rights for both sides in the format [K, Q, k, q]
    castling_rights: [bool; 4],
    /// The index of the en passant target square, 0..64
    en_passant_target: Option<usize>,
    /// The number of halfmoves since the last pawn push or capture
    halfmove_clock: usize,
    /// The number of fullmoves played
    fullmove_number: usize,
}

impl Board {
    /// Attempts to construct a `Board` from a FEN string, returning an error if the FEN is invalid
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let mut content = [Occupant::Empty; 64];
        let fields: Vec<_> = fen.split(" ").collect();
        let nfields = fields.len();
        if nfields != 6 {
            return Err(format!(
                "Invalid FEN: expected six space-separated FEN fields, got {nfields}"
            ));
        }
        let ranks: Vec<_> = fields[0].split("/").collect();
        let nranks = ranks.len();
        if nranks != 8 {
            return Err(format!(
                "Invalid FEN: expected eight ranks of pieces separated by forward-slashes, got {nranks}"
            ));
        }
        let mut wk_seen = false;
        let mut bk_seen = false;
        let mut ptr = 63;
        let mut rankn = 8;
        for rank in ranks {
            let mut rank_filled = 0;
            for piece_char in rank.chars().rev() {
                if rank_filled == 8 {
                    return Err(format!("Invalid FEN: rank {rankn} cannot have pieces beyond the h file (8 squares already occupied)"));
                }
                if piece_char.is_ascii_digit() {
                    let empty_space = piece_char.to_digit(10).unwrap() as usize;
                    if !(1..=8).contains(&empty_space) {
                        return Err(format!("Invalid FEN: {empty_space} is not a valid character for board data, digits must be in the range 1..=8"));
                    }
                    if empty_space > 8 - rank_filled {
                        return Err(format!("Invalid FEN: rank {rankn} only has 8 squares, {rank_filled} of which is/are occupied. {empty_space} more squares of empty space cannot be accomodated"));
                    }
                    rank_filled += empty_space;
                    if ptr >= empty_space {
                        ptr -= empty_space;
                    }
                } else {
                    content[ptr] = match piece_char.try_into() {
                        Ok(piece) => {
                            match piece {
                                Piece::K(true) => {
                                    if wk_seen {
                                        return Err(
                                            "Invalid FEN: white cannot have more than one king"
                                                .to_owned(),
                                        );
                                    }
                                    wk_seen = true;
                                }
                                Piece::K(false) => {
                                    if bk_seen {
                                        return Err(
                                            "Invalid FEN: black cannot have more than one king"
                                                .to_owned(),
                                        );
                                    }
                                    bk_seen = true;
                                }
                                _ => (),
                            }
                            Occupant::Piece(piece)
                        }
                        Err(e) => return Err(format!("Invalid FEN: {e}")),
                    };
                    rank_filled += 1;
                    if ptr > 0 {
                        ptr -= 1;
                    }
                }
            }
            if rank_filled != 8 {
                return Err(format!(
                    "Invalid FEN: rank {rankn} does not have data for 8 squares"
                ));
            }
            rankn -= 1;
        }
        let turn = fields[1];
        let side_to_move;
        match turn {
            "w" => side_to_move = true,
            "b" => side_to_move = false,
            _ => {
                return Err(format!(
                "Invalid FEN: Expected second field (side to move) to be 'w' or 'b', got '{turn}'"
            ))
            }
        }
        let castling = fields[2];
        let len_castling = castling.len();
        if !((1..=4).contains(&len_castling)) {
            return Err(format!("Invalid FEN: Expected third field (castling rights) to be 1 to 4 characters long, got {len_castling} characters"));
        }
        let mut castling_rights = [false; 4];
        if castling != "-" {
            for ch in castling.chars() {
                match ch {
                    'K' => {
                        if castling_rights[0] == true {
                            return Err(format!(
                                "Invalid FEN: Found more than one occurrence of 'K' in third field (castling rights)"
                            ));
                        }
                        castling_rights[0] = true;
                    }
                    'Q' => {
                        if castling_rights[1] == true {
                            return Err(format!(
                                "Invalid FEN: Found more than one occurrence of 'Q' in third field (castling rights)"
                            ));
                        }
                        castling_rights[1] = true;
                    }
                    'k' => {
                        if castling_rights[2] == true {
                            return Err(format!(
                                "Invalid FEN: Found more than one occurrence of 'k' in third field (castling rights)"
                            ));
                        }
                        castling_rights[2] = true;
                    }
                    'q' => {
                        if castling_rights[3] == true {
                            return Err(format!(
                                "Invalid FEN: Found more than one occurrence of 'q' in third field (castling rights)"
                            ));
                        }
                        castling_rights[3] = true;
                    }
                    _ => {
                        return Err(format!(
                            "Invalid FEN: Expected third field (castling rights) to contain '-' or a subset of 'KQkq', found '{ch}'"
                        ))
                    }
                }
            }
        }
        let ep = fields[3];
        let len_ep = ep.len();
        if !((1..=2).contains(&len_ep)) {
            return Err(format!("Invalid FEN: Expected fourth field (en passant target square) to be 1 to 2 characters long, got {len_ep} characters"));
        }
        let mut en_passant_target = None;
        if ep != "-" {
            let err = Err(format!(
                "Invalid FEN: Expected fourth field (en passant target square) to be '-' or a valid en passant target square name, '{ep}' is not a valid en passant target square name"
            ));
            if len_ep != 2 {
                return err;
            }
            let file = ep.chars().next().unwrap();
            let rank = ep.chars().skip(1).next().unwrap();
            if !('a' <= file && file <= 'h' && ['3', '6'].contains(&rank)) {
                return err;
            }
            en_passant_target =
                Some((rank.to_digit(10).unwrap() as usize - 1) * 8 + (file as usize - 97));
        }
        let halfmoves = fields[4];
        let halfmove_clock: usize = halfmoves.parse().map_err(|_| {
            format!(
                "Invalid FEN: Expected fifth field (halfmove clock) to be a whole number, got '{halfmoves}'"
            )
        })?;
        if halfmove_clock > 75 {
            return Err(format!("Invalid FEN: Fifth field (halfmove clock) cannot contain a value greater than 75 (the seventy-five-move rule forces a draw when this reaches 75), got {halfmove_clock}"));
        }
        let fullmoves = fields[5];
        let fullmove_number: usize = fullmoves.parse().map_err(|_| {
            format!("Invalid FEN: Expected sixth field (fullmove number) to be a natural number, got '{fullmoves}'")
        })?;
        if fullmove_number < 1 {
            return Err(format!("Invalid FEN: Sixth field (fullmove number) cannot contain a value less than 1 (it starts at 1 and increments after Black's move), got {fullmove_number}"));
        }
        Ok(Self {
            content,
            side_to_move,
            castling_rights,
            en_passant_target,
            halfmove_clock,
            fullmove_number,
        })
    }
}

impl Default for Board {
    /// Constructs a `Board` with the starting position for a chess game
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Occupant {
    Piece(Piece),
    Empty,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Piece {
    K(bool),
    Q(bool),
    B(bool),
    N(bool),
    R(bool),
    P(bool),
}

impl TryFrom<char> for Piece {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if !value.is_ascii_alphanumeric() {
            return Err(format!(
                "Invalid piece character: '{value}' is not ASCII alphanumeric"
            ));
        }
        let color = value.is_uppercase();
        Ok((match value.to_ascii_lowercase() {
            'k' => Self::K,
            'q' => Self::Q,
            'b' => Self::B,
            'n' => Self::N,
            'r' => Self::R,
            'p' => Self::P,
            _ => {
                return Err(format!(
                    "Invalid piece character: '{value}' does not correspond to any chess piece"
                ))
            }
        })(color))
    }
}

#[cfg(test)]
mod tests {
    use super::Board;

    #[test]
    fn default_board() {
        println!("{:?}", Board::default());
    }

    #[test]
    fn invalid_fen() {
        // Board::from_fen("what").unwrap();
        // Board::from_fen("blafsd o fs o sdo d").unwrap();
        // Board::from_fen("rnbkkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RKBQKBNR w KQkq - 0 1").unwrap();
        // Board::from_fen("rnbqkbnr/pppppppp/8p/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        // Board::from_fen("rnbqkbnr/pppppxpp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP-RNBQKBNR w KQkq - 0 1").unwrap();
        // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR lol KQkq - 0 1").unwrap();
        // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b ros - 0 1").unwrap();
        // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KKqk - 0 1").unwrap();
        // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq C6 0 1").unwrap();
        // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq c5 0 1").unwrap();
        // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq brr 0 1").unwrap();
        // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0.1 1").unwrap();
        // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 76 1").unwrap();
        // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 75 0").unwrap();
        // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 75 bro").unwrap();
        println!(
            "{:?}",
            Board::from_fen("6k1/8/6K1/6P1/8/8/8/8 w - - 0 1").unwrap()
        );
    }
}

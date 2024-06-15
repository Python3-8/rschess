use super::{helpers, Color, IllegalMoveError, InvalidSanMoveError, Move, Piece, PieceType, SpecialMoveType};
use std::{
    collections::HashMap,
    fmt,
    sync::{Mutex, OnceLock},
};

/// Returns the cached positions and their legal moves.
fn legal_move_cache() -> &'static Mutex<HashMap<Position, Vec<Move>>> {
    static LEGAL_MOVE_CACHE: OnceLock<Mutex<HashMap<Position, Vec<Move>>>> = OnceLock::new();
    LEGAL_MOVE_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// The structure for a chess position
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Position {
    /// The board content; each square is represented by a number 0..64 where a1 is 0, h1 is 7, and h8 is 63
    pub(crate) content: [Option<Piece>; 64],
    /// The side to move; white is `true` and black is `false`
    pub(crate) side: Color,
    /// The indices of rook locations representing castling rights for both sides in the format [K, Q, k, q]
    pub(crate) castling_rights: [Option<usize>; 4],
    /// The index of the en passant target square, 0..64
    pub(crate) ep_target: Option<usize>,
}

impl Position {
    /// Generates an FEN string representing the board data, active color, castling rights, and en passant target in the position.
    pub fn to_fen(&self) -> String {
        let Self {
            content,
            side,
            castling_rights,
            ep_target,
        } = self;
        let mut rankstrs = Vec::new();
        for rank in content.chunks(8).rev() {
            let mut rankstr = String::new();
            let mut empty = 0;
            for sq in rank {
                match sq {
                    Some(p) => {
                        if empty > 0 {
                            rankstr.push(char::from_digit(empty, 10).unwrap());
                            empty = 0;
                        }
                        rankstr.push((*p).into());
                    }
                    None => {
                        empty += 1;
                    }
                }
            }
            if empty > 0 {
                rankstr.push(char::from_digit(empty, 10).unwrap());
            }
            rankstrs.push(rankstr);
        }
        let board_data = rankstrs.join("/");
        let active_color = char::from(*side).to_string();
        let mut castling_availability = String::new();
        let count_rooks = |rng, color| helpers::count_piece(rng, Piece(PieceType::R, color), content);
        let (wk, bk) = (helpers::find_king(Color::White, content), helpers::find_king(Color::Black, content));
        if castling_rights[0].is_some() {
            castling_availability.push(if count_rooks(wk + 1..8, Color::White) == 1 {
                'K'
            } else {
                helpers::idx_to_sq(castling_rights[0].unwrap()).0.to_ascii_uppercase()
            });
        }
        if castling_rights[1].is_some() {
            castling_availability.push(if count_rooks(0..wk, Color::White) == 1 {
                'Q'
            } else {
                helpers::idx_to_sq(castling_rights[1].unwrap()).0.to_ascii_uppercase()
            });
        }
        if castling_rights[2].is_some() {
            castling_availability.push(if count_rooks(bk + 1..64, Color::Black) == 1 {
                'k'
            } else {
                helpers::idx_to_sq(castling_rights[2].unwrap()).0
            });
        }
        if castling_rights[3].is_some() {
            castling_availability.push(if count_rooks(56..bk, Color::Black) == 1 {
                'q'
            } else {
                helpers::idx_to_sq(castling_rights[2].unwrap()).0
            });
        }
        if castling_availability.is_empty() {
            castling_availability.push('-');
        }
        let en_passant_target_square;
        if let Some(target) = ep_target {
            let (f, r) = helpers::idx_to_sq(*target);
            en_passant_target_square = [f.to_string(), r.to_string()].join("");
        } else {
            en_passant_target_square = "-".to_owned();
        }
        [board_data, active_color, castling_availability, en_passant_target_square].join(" ")
    }

    /// Converts a `Move` to SAN, returning an error if the move is illegal.
    pub fn move_to_san(&self, move_: Move) -> Result<String, IllegalMoveError> {
        let legal = self.gen_non_illegal_moves();
        let move_ = match helpers::as_legal(move_, &legal) {
            Some(m) => m,
            _ => return Err(IllegalMoveError(move_)),
        };
        let mut san = String::new();
        let Move(src, dest, spec) = move_;
        let Self { content, .. } = self;
        let (src_occ, dest_occ) = (content[src], content[dest]);
        let ((srcf, srcr), (destf, destr)) = (helpers::idx_to_sq(src), helpers::idx_to_sq(dest));
        let new_content = self.make_move(move_).unwrap();
        let suffix = if new_content.is_checkmate() {
            "#"
        } else if new_content.is_check() {
            "+"
        } else {
            ""
        };
        let piece_type;
        match src_occ {
            Some(Piece(pt, _)) => match pt {
                PieceType::P => {
                    return Ok(format!(
                        "{}{suffix}",
                        match spec {
                            Some(SpecialMoveType::EnPassant) => format!("{srcf}x{destf}{destr}"),
                            _ => format!(
                                "{}{}",
                                match dest_occ {
                                    Some(_) => format!("{srcf}x{destf}{destr}",),
                                    None => format!("{destf}{destr}"),
                                },
                                match spec {
                                    Some(SpecialMoveType::Promotion(piece_type)) => format!("={}", char::from(piece_type)),
                                    _ => String::new(),
                                }
                            ),
                        },
                    ))
                }
                PieceType::K => {
                    return Ok(format!(
                        "{}{suffix}",
                        match spec {
                            Some(SpecialMoveType::CastlingKingside) => "O-O".to_owned(),
                            Some(SpecialMoveType::CastlingQueenside) => "O-O-O".to_owned(),
                            _ => format!(
                                "K{}{destf}{destr}",
                                match dest_occ {
                                    Some(_) => "x",
                                    None => "",
                                }
                            ),
                        },
                    ))
                }
                pt => {
                    san.push(char::from(pt));
                    piece_type = pt;
                }
            },
            _ => panic!("the universe is malfunctioning"),
        }
        if legal
            .iter()
            .filter(|m| {
                if m.1 == dest {
                    if let Some(Piece(pt, _)) = content[m.0] {
                        pt == piece_type
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
            .count()
            > 1
        {
            if legal
                .iter()
                .filter(|m| {
                    if m.1 == dest {
                        if let Some(Piece(pt, _)) = content[m.0] {
                            pt == piece_type && helpers::squares_in_file(srcf).contains(&m.0)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                })
                .count()
                > 1
            {
                if legal
                    .iter()
                    .filter(|m| {
                        if m.1 == dest {
                            if let Some(Piece(pt, _)) = content[m.0] {
                                pt == piece_type && helpers::squares_in_rank(srcr).contains(&m.0)
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    })
                    .count()
                    > 1
                {
                    san.push(srcf);
                }
                san.push(srcr);
            } else {
                san.push(srcf);
            }
        }
        Ok(format!(
            "{san}{}{destf}{destr}{suffix}",
            match dest_occ {
                Some(_) => "x",
                None => "",
            }
        ))
    }

    /// Constructs a `Move` from a SAN representation, returning an error if it is invalid or illegal.
    pub fn san_to_move(&self, san: &str) -> Result<Move, InvalidSanMoveError> {
        let san = san.replace('0', "O").replace(['+', '#'], "");
        self.gen_non_illegal_moves()
            .into_iter()
            .find(|&m| self.move_to_san(m).unwrap().replace(['+', '#'], "") == san)
            .ok_or(InvalidSanMoveError(san.to_owned()))
    }

    /// Returns the position which would occur if the given move is played, returning an error if the move is illegal.
    pub fn make_move(&self, move_: Move) -> Result<Self, IllegalMoveError> {
        let move_ = match helpers::as_legal(move_, &self.gen_non_illegal_moves()) {
            Some(m) => m,
            _ => return Err(IllegalMoveError(move_)),
        };
        let castling_rights_idx_offset = if self.side.is_white() { 0 } else { 2 };
        let Self {
            content,
            mut side,
            mut castling_rights,
            ..
        } = self;
        let mut ep_target = None;
        let Move(move_src, move_dest, ..) = move_;
        let moved_piece = content[move_src];
        match moved_piece {
            Some(Piece(PieceType::K, _)) => (castling_rights[castling_rights_idx_offset], castling_rights[castling_rights_idx_offset + 1]) = (None, None),
            Some(Piece(PieceType::P, _)) => {
                if (std::cmp::max(move_src, move_dest) - std::cmp::min(move_src, move_dest)) == 16 {
                    ep_target = Some(if side.is_white() { move_src + 8 } else { move_src - 8 });
                }
            }
            _ => (),
        }
        for maybe_rook in [move_src, move_dest] {
            let maybe_right = castling_rights.iter().enumerate().find(|(_, right)| right.is_some() && right.unwrap() == maybe_rook);
            if maybe_right.is_some() {
                castling_rights[maybe_right.unwrap().0] = None;
            }
        }
        side = !side;
        let new_content = helpers::change_content(content, &move_, &self.castling_rights);
        Ok(Self {
            content: new_content,
            side,
            castling_rights,
            ep_target,
        })
    }

    /// Pretty-prints the position to a string, from the perspective of the side `perspective`.
    pub fn pretty_print(&self, perspective: Color) -> String {
        let mut string = String::new();
        if perspective.is_white() {
            for (ranki, rank) in self.content.chunks(8).rev().enumerate() {
                string += &format!("{} |", 8 - ranki);
                for (sqi, occupant) in rank.iter().enumerate() {
                    string += &format!(" {} ", if let Some(p) = occupant { format!("{p}").chars().next().unwrap() } else { ' ' });
                    if sqi != 7 {
                        string.push('|');
                    }
                }
                string.push('\n');
                string += &"—".repeat(33);
                string.push('\n');
            }
            string += "  | a | b | c | d | e | f | g | h";
        } else {
            for (ranki, rank) in self.content.chunks(8).enumerate() {
                string += &format!("{} |", ranki + 1);
                for (sqi, occupant) in rank.iter().rev().enumerate() {
                    string += &format!(" {} ", if let Some(p) = occupant { format!("{p}").chars().next().unwrap() } else { ' ' });
                    if sqi != 7 {
                        string.push('|');
                    }
                }
                string.push('\n');
                string += &"—".repeat(33);
                string.push('\n');
            }
            string += "  | h | g | f | e | d | c | d | a";
        }
        string
    }

    /// Generates the legal moves in the position, assuming the game is ongoing.
    pub fn gen_non_illegal_moves(&self) -> Vec<Move> {
        if let Some(v) = legal_move_cache().lock().unwrap().get(self) {
            return v.clone();
        }
        let Self { content, side, castling_rights, .. } = self;
        let v: Vec<_> = self
            .gen_pseudolegal_moves()
            .into_iter()
            .filter(|move_| {
                if let Move(src, dest, Some(SpecialMoveType::CastlingKingside | SpecialMoveType::CastlingQueenside)) = move_ {
                    for sq in *std::cmp::min(src, dest)..=*std::cmp::max(src, dest) {
                        if self.controls_square(sq, !*side) {
                            return false;
                        }
                    }
                    return true;
                }
                !helpers::king_capture_pseudolegal(&helpers::change_content(content, move_, castling_rights), !*side)
            })
            .collect();
        legal_move_cache().lock().unwrap().insert(self.clone(), v.clone());
        v
    }

    /// Checks whether the game is drawn by stalemate. Use [`Position::stalemated_side`] to know which side is in stalemate.
    pub fn is_stalemate(&self) -> bool {
        !self.is_check() && self.gen_non_illegal_moves().is_empty()
    }

    /// Checks whether any side is in check (a checkmate is also considered a check). Use [`Position::checked_side`] to know which side is in check.
    pub fn is_check(&self) -> bool {
        self.checked_side().is_some()
    }

    /// Checks whether any side is in checkmate. Use [`Position::checkmated_side`] to know which side is in checkmate.
    pub fn is_checkmate(&self) -> bool {
        self.is_check() && self.gen_non_illegal_moves().is_empty()
    }

    /// Returns an optional boolean representing the side in stalemate (`None` if neither side is in stalemate).
    pub fn stalemated_side(&self) -> Option<Color> {
        if self.is_stalemate() {
            Some(self.side)
        } else {
            None
        }
    }

    /// Returns an optional boolean representing the side in check (`None` if neither side is in check).
    pub fn checked_side(&self) -> Option<Color> {
        if helpers::king_capture_pseudolegal(&self.content, Color::Black) {
            Some(Color::White)
        } else if helpers::king_capture_pseudolegal(&self.content, Color::White) {
            Some(Color::Black)
        } else {
            None
        }
    }

    /// Returns an optional boolean representing the side in checkmate (`None` if neither side is in checkmate).
    pub fn checkmated_side(&self) -> Option<Color> {
        if self.is_checkmate() {
            Some(self.side)
        } else {
            None
        }
    }

    /// Generates the pseudolegal moves in the position.
    pub(crate) fn gen_pseudolegal_moves(&self) -> Vec<Move> {
        let Self {
            content,
            castling_rights,
            ep_target,
            side,
        } = self;
        let mut pseudolegal_moves = Vec::new();
        for (i, sq) in content.iter().enumerate() {
            if let Some(piece) = sq {
                if piece.1 != *side {
                    continue;
                }
                match piece.0 {
                    PieceType::K => {
                        let mut possible_dests = Vec::new();
                        for axis in [1, 8, 7, 9] {
                            if helpers::long_range_can_move(i, axis as isize) {
                                possible_dests.push(i + axis);
                            }
                            if helpers::long_range_can_move(i, -(axis as isize)) {
                                possible_dests.push(i - axis);
                            }
                        }
                        possible_dests.retain(|&dest| match content[dest] {
                            Some(Piece(_, color)) => color != *side,
                            _ => true,
                        });
                        pseudolegal_moves.extend(possible_dests.into_iter().map(|d| Move(i, d, None)));
                        let castling_rights_idx_offset = if side.is_white() { 0 } else { 2 };
                        let (oo_sq, ooo_sq) = if side.is_white() { (6, 2) } else { (62, 58) };
                        let (kingside, queenside) = (castling_rights[castling_rights_idx_offset], castling_rights[castling_rights_idx_offset + 1]);
                        if let Some(r) = kingside {
                            match helpers::count_pieces(i + 1..=oo_sq, content) {
                                0 => pseudolegal_moves.push(Move(i, oo_sq, Some(SpecialMoveType::CastlingKingside))),
                                1 => {
                                    if helpers::find_all_pieces(i + 1..=oo_sq, content)[0] == r {
                                        pseudolegal_moves.push(Move(i, oo_sq, Some(SpecialMoveType::CastlingKingside)))
                                    }
                                }
                                _ => (),
                            }
                        }
                        if let Some(r) = queenside {
                            match helpers::count_pieces(ooo_sq..i, content) {
                                0 => pseudolegal_moves.push(Move(i, ooo_sq, Some(SpecialMoveType::CastlingQueenside))),
                                1 => {
                                    if helpers::find_all_pieces(ooo_sq..i, content)[0] == r {
                                        pseudolegal_moves.push(Move(i, ooo_sq, Some(SpecialMoveType::CastlingQueenside)))
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    PieceType::N => {
                        let b_r_axes = [(7, [-1, 8]), (9, [8, 1]), (-7, [1, -8]), (-9, [-8, -1])];
                        let mut dest_squares = Vec::new();
                        for (b_axis, r_axes) in b_r_axes {
                            if !helpers::long_range_can_move(i, b_axis) {
                                continue;
                            }
                            let b_dest = i as isize + b_axis;
                            for r_axis in r_axes {
                                if !helpers::long_range_can_move(b_dest as usize, r_axis) {
                                    continue;
                                }
                                dest_squares.push((b_dest + r_axis) as usize);
                            }
                        }
                        pseudolegal_moves.extend(
                            dest_squares
                                .into_iter()
                                .filter(|&dest| match content[dest] {
                                    Some(Piece(_, color)) => color != *side,
                                    _ => true,
                                })
                                .map(|dest| Move(i, dest, None)),
                        )
                    }
                    PieceType::P => {
                        let mut possible_dests = Vec::new();
                        if side.is_white() {
                            if content[i + 8].is_none() {
                                possible_dests.push((i + 8, false));
                                if (8..16).contains(&i) && content[i + 16].is_none() {
                                    possible_dests.push((i + 16, false))
                                }
                            }
                            if helpers::long_range_can_move(i, 7) {
                                if let Some(Piece(_, color)) = content[i + 7] {
                                    if color.is_black() {
                                        possible_dests.push((i + 7, false));
                                    }
                                } else if ep_target.is_some() && ep_target.unwrap() == i + 7 {
                                    possible_dests.push((i + 7, true));
                                }
                            }
                            if helpers::long_range_can_move(i, 9) {
                                if let Some(Piece(_, color)) = content[i + 9] {
                                    if color.is_black() {
                                        possible_dests.push((i + 9, false));
                                    }
                                } else if ep_target.is_some() && ep_target.unwrap() == i + 9 {
                                    possible_dests.push((i + 9, true));
                                }
                            }
                        } else {
                            if content[i - 8].is_none() {
                                possible_dests.push((i - 8, false));
                                if (48..56).contains(&i) && content[i - 16].is_none() {
                                    possible_dests.push((i - 16, false))
                                }
                            }
                            if helpers::long_range_can_move(i, -9) {
                                if let Some(Piece(_, color)) = content[i - 9] {
                                    if color.is_white() {
                                        possible_dests.push((i - 9, false));
                                    }
                                } else if ep_target.is_some() && ep_target.unwrap() == i - 9 {
                                    possible_dests.push((i - 9, true));
                                }
                            }
                            if helpers::long_range_can_move(i, -7) {
                                if let Some(Piece(_, color)) = content[i - 7] {
                                    if color.is_white() {
                                        possible_dests.push((i - 7, false));
                                    }
                                } else if ep_target.is_some() && ep_target.unwrap() == i - 7 {
                                    possible_dests.push((i - 7, true));
                                }
                            }
                        }
                        pseudolegal_moves.extend(possible_dests.into_iter().flat_map(|(dest, ep)| {
                            if (0..8).contains(&dest) || (56..64).contains(&dest) {
                                [PieceType::Q, PieceType::R, PieceType::B, PieceType::N]
                                    .into_iter()
                                    .map(|p| Move(i, dest, Some(SpecialMoveType::Promotion(p))))
                                    .collect()
                            } else {
                                vec![Move(i, dest, if ep { Some(SpecialMoveType::EnPassant) } else { None })]
                            }
                        }));
                    }
                    long_range_type => pseudolegal_moves.append(&mut self.gen_long_range_piece_pseudolegal_moves(i, long_range_type)),
                }
            }
        }
        pseudolegal_moves
    }

    /// Generates pseudolegal moves for a long-range piece.
    pub(crate) fn gen_long_range_piece_pseudolegal_moves(&self, sq: usize, piece_type: PieceType) -> Vec<Move> {
        let Self { content, side, .. } = self;
        let axes = match piece_type {
            PieceType::Q => vec![1, 8, 7, 9],
            PieceType::R => vec![1, 8],
            PieceType::B => vec![7, 9],
            _ => panic!("not a long-range piece"),
        };
        let mut dest_squares = Vec::new();
        for axis in axes {
            'axis: for axis_direction in [-axis, axis] {
                let mut current_sq = sq as isize;
                while helpers::long_range_can_move(current_sq as usize, axis_direction) {
                    let mut skip = false;
                    current_sq += axis_direction;
                    if let Some(Piece(_, color)) = content[current_sq as usize] {
                        if color == *side {
                            continue 'axis;
                        } else {
                            skip = true;
                        }
                    }
                    dest_squares.push(current_sq as usize);
                    if skip {
                        continue 'axis;
                    }
                }
            }
        }
        dest_squares.into_iter().map(|dest| Move(sq, dest, None)).collect()
    }

    /// Checks whether the given side controls a specified square in this position.
    pub(crate) fn controls_square(&self, sq: usize, side: Color) -> bool {
        let Self {
            mut content,
            castling_rights,
            ep_target,
            ..
        } = self.clone();
        content[sq] = Some(Piece(PieceType::P, !side));
        Self {
            content,
            side,
            castling_rights,
            ep_target,
        }
        .gen_pseudolegal_moves()
        .into_iter()
        .any(|Move(_, dest, _)| dest == sq)
    }

    /// Counts the material on the board. This function is used by [`Position::is_insufficient_material`] to determine whether there is insufficient checkmating material.
    pub(crate) fn count_material(&self) -> Vec<Material> {
        let mut material = Vec::new();
        for sq in 0..64 {
            if let Some(Piece(piece_type, _)) = self.content[sq] {
                match piece_type {
                    PieceType::K => (),
                    PieceType::N => material.push(Material::Knight),
                    PieceType::B => material.push(Material::Bishop(helpers::color_complex_of(sq))),
                    _ => material.push(Material::Other),
                }
            }
        }
        material
    }

    /// Checks whether the game is drawn by insufficient material.
    pub fn is_insufficient_material(&self) -> bool {
        let copy1 = self.count_material();
        let (mut copy2, copy3, mut copy4) = (copy1.clone(), copy1.clone(), copy1.clone());
        if copy1.is_empty() {
            return true;
        }
        for (i, m) in copy2.iter().enumerate() {
            if let Material::Knight = m {
                copy2.remove(i);
                break;
            }
        }
        if copy2.is_empty() {
            return true;
        }
        let mut b_complex = None;
        for m in copy3.iter() {
            if let Material::Bishop(complex) = m {
                b_complex = Some(complex);
                break;
            }
        }
        if let Some(complex) = b_complex {
            copy4.retain(|m| m != &Material::Bishop(*complex));
            if copy4.is_empty() {
                return true;
            }
        }
        false
    }

    /// Returns which side's turn it is to move.
    pub fn side_to_move(&self) -> Color {
        self.side
    }

    /// Checks whether the given move is a capture, returning an error if it is illegal in this position.
    pub fn is_capture(&self, move_: Move) -> Result<bool, IllegalMoveError> {
        let move_ = match helpers::as_legal(move_, &self.gen_non_illegal_moves()) {
            Some(m) => m,
            _ => return Err(IllegalMoveError(move_)),
        };
        Ok(move_.2 == Some(SpecialMoveType::EnPassant) || self.content[move_.1].is_some())
    }
}

impl fmt::Display for Position {
    /// Pretty-prints the position from the perspective of the side whose turn it is to move.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pretty_print(self.side))
    }
}

/// Represents a piece of material.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Material {
    Knight,
    Bishop(bool),
    Other,
}

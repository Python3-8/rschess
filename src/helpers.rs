use super::{Color, Move, Piece, PieceType, Position, SpecialMoveType};
use std::ops::RangeBounds;

/// Converts a square name in the format (<file>, <rank>) to a square index.
pub fn sq_to_idx(file: char, rank: char) -> usize {
    (rank.to_digit(10).unwrap() as usize - 1) * 8 + (file as usize - 97)
}

/// Converts a square index to a square name in the format (<file>, <rank>).
pub fn idx_to_sq(idx: usize) -> (char, char) {
    ((idx % 8 + 97) as u8 as char, char::from_digit((idx / 8 + 1) as u32, 10).unwrap())
}

/// Checks whether a long-range piece can move on the axis `axis_direction` from the square `sq`
pub fn long_range_can_move(sq: usize, axis_direction: isize) -> bool {
    !(axis_direction == 1 && (sq + 1) % 8 == 0
        || axis_direction == -1 && sq % 8 == 0
        || axis_direction == 8 && sq >= 56
        || axis_direction == -8 && sq < 8
        || axis_direction == 7 && (sq >= 56 || sq % 8 == 0)
        || axis_direction == -7 && (sq < 8 || (sq + 1) % 8 == 0)
        || axis_direction == 9 && (sq >= 56 || (sq + 1) % 8 == 0)
        || axis_direction == -9 && (sq < 8 || sq % 8 == 0))
}

/// Counts the number of pieces on the board identical to the `piece` provided that are within the provided square range.
pub fn count_piece<R>(rng: R, piece: Piece, content: &[Option<Piece>; 64]) -> usize
where
    R: RangeBounds<usize> + Iterator<Item = usize>,
{
    find_pieces(piece, rng, content).len()
}

/// Counts the number of pieces on the board that are within the provided square range.
pub fn count_pieces<R>(rng: R, content: &[Option<Piece>; 64]) -> usize
where
    R: RangeBounds<usize> + Iterator<Item = usize>,
{
    rng.fold(0, |acc, sq| if content[sq].is_some() { acc + 1 } else { acc })
}

/// Finds the indices of all occurrences of a piece identical to the given `piece` on the board in the square range `rng`.
pub fn find_pieces<R>(piece: Piece, rng: R, content: &[Option<Piece>; 64]) -> Vec<usize>
where
    R: RangeBounds<usize> + Iterator<Item = usize>,
{
    let piece = Some(piece);
    rng.filter(|&sq| content[sq] == piece).collect()
}

/// Finds the indices of all occurrences pieces on the board in the square range `rng`.
pub fn find_all_pieces<R>(rng: R, content: &[Option<Piece>; 64]) -> Vec<usize>
where
    R: RangeBounds<usize> + Iterator<Item = usize>,
{
    rng.filter(|&sq| content[sq].is_some()).collect()
}

/// Checks whether capturing a king is pseudolegal for the specified side in the given position.
pub fn king_capture_pseudolegal(content: &[Option<Piece>; 64], side: Color) -> bool {
    let enemy_king = find_king(!side, content);
    Position {
        content: *content,
        side,
        castling_rights: [None, None, None, None],
        ep_target: None,
    }
    .controls_square(enemy_king, side)
}

/// Returns the square index of the king of color `color`.
pub fn find_king(color: Color, content: &[Option<Piece>; 64]) -> usize {
    content
        .iter()
        .enumerate()
        .find(|(_, o)| if let Some(Piece(PieceType::K, s)) = o { *s == color } else { false })
        .unwrap()
        .0
}

/// Changes the board content based on the given move.
pub fn change_content(content: &[Option<Piece>; 64], move_: &Move, castling_rights: &[Option<usize>]) -> [Option<Piece>; 64] {
    let mut content = *content;
    let Move(src, dest, spec) = move_;
    (content[*src], content[*dest]) = (None, content[*src]);
    match spec {
        Some(SpecialMoveType::CastlingKingside | SpecialMoveType::CastlingQueenside) => match *dest {
            6 => {
                let krook = castling_rights[0].unwrap();
                (content[krook], content[5]) = (None, content[krook]);
            }
            2 => {
                let qrook = castling_rights[1].unwrap();
                (content[qrook], content[3]) = (None, content[qrook]);
            }
            62 => {
                let krook = castling_rights[2].unwrap();
                (content[krook], content[61]) = (None, content[krook]);
            }
            58 => {
                let qrook = castling_rights[3].unwrap();
                (content[qrook], content[59]) = (None, content[qrook]);
            }
            _ => panic!("the universe is malfunctioning"),
        },
        Some(SpecialMoveType::EnPassant) => match dest {
            16..=23 => content[dest + 8] = None,
            40..=47 => content[dest - 8] = None,
            _ => panic!("the universe is malfunctioning"),
        },
        Some(SpecialMoveType::Promotion(piece_type)) => {
            if let Some(Piece(_, color)) = content[*dest] {
                content[*dest] = Some(Piece(*piece_type, color));
            }
        }
        _ => (),
    }
    content
}

/// Checks whether `sq` is a light square.
pub fn color_complex_of(sq: usize) -> bool {
    (match sq {
        0..=7 | 16..=23 | 32..=39 | 48..=55 => sq + 1,
        _ => sq,
    }) % 2
        == 0
}

/// Returns a list of the indices of all the squares in a file.
pub fn squares_in_file(file: char) -> Vec<usize> {
    let mut vec = Vec::new();
    let bottom = sq_to_idx(file, '1');
    for i in 0..8 {
        vec.push(bottom + 8 * i);
    }
    vec
}

/// Returns a list of the indices of all the squares on a rank.
pub fn squares_in_rank(rank: char) -> Vec<usize> {
    let mut vec = Vec::new();
    let left = 8 * (rank.to_digit(10).unwrap() as usize - 1);
    for i in 0..8 {
        vec.push(left + i);
    }
    vec
}

pub fn as_legal(move_: Move, legal: &[Move]) -> Option<Move> {
    if legal.contains(&move_) {
        Some(move_)
    } else if move_.2 == Some(SpecialMoveType::Unclear) {
        match legal.iter().find(|m| (m.0, m.1) == (move_.0, move_.1) && !matches!(m.2, Some(SpecialMoveType::Promotion(_)))) {
            Some(&m) => Some(m),
            _ => None,
        }
    } else {
        None
    }
}

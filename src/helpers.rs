use super::{Move, Occupant, Piece, PieceType, Position};
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
pub fn count_piece<R>(rng: R, piece: Piece, content: &[Occupant; 64]) -> usize
where
    R: RangeBounds<usize> + Iterator<Item = usize>,
{
    find_pieces(piece, rng, content).len()
}

/// Counts the number of pieces on the board that are within the provided square range.
pub fn count_pieces<R>(rng: R, content: &[Occupant; 64]) -> usize
where
    R: RangeBounds<usize> + Iterator<Item = usize>,
{
    rng.fold(0, |acc, sq| if let Occupant::Piece(_) = content[sq] { acc + 1 } else { acc })
}

/// Finds the indices of all occurrences of a piece identical to the given `piece` on the board in the square range `rng`.
pub fn find_pieces<R>(piece: Piece, rng: R, content: &[Occupant; 64]) -> Vec<usize>
where
    R: RangeBounds<usize> + Iterator<Item = usize>,
{
    let piece = Occupant::Piece(piece);
    rng.filter(|&sq| content[sq] == piece).collect()
}

/// Checks whether capturing a king is pseudolegal for the specified side in the given position.
pub fn king_capture_pseudolegal(content: &[Occupant; 64], side: bool) -> bool {
    let enemy_king = content
        .into_iter()
        .enumerate()
        .find(|(_, o)| {
            if let Occupant::Piece(Piece(PieceType::K, s)) = o {
                if *s != side {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        })
        .unwrap()
        .0;
    Position {
        content: content.clone(),
        side,
        castling_rights: [false, false, false, false],
        ep_target: None,
    }
    .controls_square(enemy_king, side)
}

/// Changes the board content based on the given move.
pub fn change_content(content: &[Occupant; 64], move_: &Move) -> [Occupant; 64] {
    let mut content = content.clone();
    let Move(src, dest, spec) = move_;
    (content[*src], content[*dest]) = (Occupant::Empty, content[*src]);
    match spec {
        Some(PieceType::K) => match *dest {
            6 => {
                let rook = Piece(PieceType::R, true);
                let krook = find_pieces(rook, src + 1..=6, &content)[0];
                (content[krook], content[5]) = (Occupant::Empty, content[krook]);
            }
            2 => {
                let rook = Piece(PieceType::R, true);
                let qrook = find_pieces(rook, 0..*src, &content)[0];
                (content[qrook], content[3]) = (Occupant::Empty, content[qrook]);
            }
            62 => {
                let rook = Piece(PieceType::R, false);
                let krook = find_pieces(rook, src + 1..=62, &content)[0];
                (content[krook], content[61]) = (Occupant::Empty, content[krook]);
            }
            58 => {
                let rook = Piece(PieceType::R, false);
                let qrook = find_pieces(rook, 56..*src, &content)[0];
                (content[qrook], content[59]) = (Occupant::Empty, content[qrook]);
            }
            _ => panic!("the universe is malfunctioning"),
        },
        Some(PieceType::P) => match dest {
            16..=23 => content[dest + 8] = Occupant::Empty,
            40..=47 => content[dest - 8] = Occupant::Empty,
            _ => panic!("the universe is malfunctioning"),
        },
        Some(piece_type) => {
            if let Occupant::Piece(Piece(_, color)) = content[*dest] {
                content[*dest] = Occupant::Piece(Piece(*piece_type, color));
            }
        }
        _ => (),
    }
    content
}

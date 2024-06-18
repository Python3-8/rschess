//! A Rust chess library with the aim to be as feature-rich as possible
//!
//! Examples are available on the [GitHub repository page](https://github.com/Python3-8/rschess).

mod board;
pub mod errors;
mod fen;
mod helpers;
#[cfg(feature = "img")]
pub mod img;
#[cfg(feature = "pgn")]
pub mod pgn;
mod position;

pub use board::Board;
pub(crate) use errors::*;
pub use fen::Fen;
pub use position::Position;
use std::{collections::HashMap, fmt, ops::Not};

/// Converts a square index (`0..64`) to a square name, returning an error if the square index is invalid.
pub fn idx_to_sq(idx: usize) -> Result<(char, char), InvalidSquareIndexError> {
    if !(0..64).contains(&idx) {
        return Err(InvalidSquareIndexError(idx));
    }
    Ok(helpers::idx_to_sq(idx))
}

/// Converts a square name to a square index, returning an error if the square name is invalid.
pub fn sq_to_idx(file: char, rank: char) -> Result<usize, InvalidSquareNameError> {
    if !(('a'..'h').contains(&file) && ('1'..'8').contains(&rank)) {
        return Err(InvalidSquareNameError(file, rank));
    }
    Ok(helpers::sq_to_idx(file, rank))
}

/// Represents a piece in the format (_piece type_, _color_).
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Piece(PieceType, Color);

impl Piece {
    /// Returns the type of piece.
    pub fn piece_type(&self) -> PieceType {
        self.0
    }

    /// Returns the color of the piece.
    pub fn color(&self) -> Color {
        self.1
    }
}

impl TryFrom<char> for Piece {
    type Error = InvalidPieceCharacterError;

    /// Attempts to convert a piece character to a `Piece`.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(Self(PieceType::try_from(value)?, if value.is_ascii_uppercase() { Color::White } else { Color::Black }))
    }
}

impl From<Piece> for char {
    /// Converts a `Piece` to a piece character.
    fn from(piece: Piece) -> char {
        let ch = piece.0.into();
        match piece.1 {
            Color::White => ch,
            Color::Black => ch.to_ascii_lowercase(),
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let codepoints = HashMap::from([
            (PieceType::K, 0x2654),
            (PieceType::Q, 0x2655),
            (PieceType::R, 0x2656),
            (PieceType::B, 0x2657),
            (PieceType::N, 0x2658),
            (PieceType::P, 0x2659),
        ]);
        let Self(t, c) = self;
        write!(f, "{}", char::from_u32((codepoints.get(t).unwrap() + if c.is_white() { 0 } else { 6 }) as u32).unwrap())
    }
}

/// Represents types of pieces.
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum PieceType {
    K,
    Q,
    B,
    N,
    R,
    P,
}

impl TryFrom<char> for PieceType {
    type Error = InvalidPieceCharacterError;

    /// Attempts to convert a piece character to a `PieceType`.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        if !value.is_ascii_alphanumeric() {
            return Err(InvalidPieceCharacterError(value));
        }
        Ok(match value.to_ascii_lowercase() {
            'k' => Self::K,
            'q' => Self::Q,
            'b' => Self::B,
            'n' => Self::N,
            'r' => Self::R,
            'p' => Self::P,
            _ => return Err(InvalidPieceCharacterError(value)),
        })
    }
}

impl From<PieceType> for char {
    /// Converts a `PieceType` to a piece character.
    fn from(piece_type: PieceType) -> char {
        match piece_type {
            PieceType::K => 'K',
            PieceType::Q => 'Q',
            PieceType::B => 'B',
            PieceType::N => 'N',
            PieceType::R => 'R',
            PieceType::P => 'P',
        }
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

/// The structure for a chess move, in the format (_source square_, _destination square_, _castling/promotion/en passant_)
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Move(usize, usize, Option<SpecialMoveType>);

impl Move {
    /// Returns the source square of the move in the format (_file_, _rank_).
    pub fn from_square(&self) -> (char, char) {
        helpers::idx_to_sq(self.0)
    }

    /// Returns the destination square of the move in the format (_file_, _rank_).
    pub fn to_square(&self) -> (char, char) {
        helpers::idx_to_sq(self.1)
    }

    /// Returns the type of special move (castling/promotion/en passant) if this move is a special move (otherwise `None`).
    pub fn special_move_type(&self) -> Option<SpecialMoveType> {
        self.2
    }

    /// Creates a `Move` object from its UCI representation.
    pub fn from_uci(uci: &str) -> Result<Self, InvalidUciError> {
        let uci_len = uci.len();
        if ![4, 5].contains(&uci_len) {
            return Err(InvalidUciError::Length);
        }
        let from_square = (uci.chars().next().unwrap(), uci.chars().nth(1).unwrap());
        let to_square = (uci.chars().nth(2).unwrap(), uci.chars().nth(3).unwrap());
        let promotion = uci.chars().nth(4);
        if !(('a'..='h').contains(&from_square.0) && ('1'..='8').contains(&from_square.1)) {
            return Err(InvalidUciError::InvalidSquareName(from_square.0, from_square.1));
        }
        if !(('a'..='h').contains(&to_square.0) && ('1'..='8').contains(&to_square.1)) {
            return Err(InvalidUciError::InvalidSquareName(to_square.0, to_square.1));
        }
        let (src, dest) = (helpers::sq_to_idx(from_square.0, from_square.1), helpers::sq_to_idx(to_square.0, to_square.1));
        let promotion = match promotion {
            Some(p) => Some({
                let pt = PieceType::try_from(p).map_err(|_| InvalidUciError::InvalidPieceType(p))?;
                if pt == PieceType::K {
                    return Err(InvalidUciError::InvalidPieceType(p));
                } else {
                    pt
                }
            }),
            _ => None,
        };
        Ok(Self(
            src,
            dest,
            match promotion {
                Some(p) => Some(SpecialMoveType::Promotion(p)),
                _ => Some(SpecialMoveType::Unclear),
            },
        ))
    }

    /// Returns the UCI representation of the move.
    pub fn to_uci(&self) -> String {
        let ((srcf, srcr), (destf, destr)) = (helpers::idx_to_sq(self.0), helpers::idx_to_sq(self.1));
        format!(
            "{srcf}{srcr}{destf}{destr}{}",
            match self.2 {
                Some(SpecialMoveType::Promotion(pt)) => char::from(pt).to_ascii_lowercase().to_string(),
                _ => String::new(),
            }
        )
    }
}

impl fmt::Display for Move {
    /// Converts the move to a UCI string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_uci())
    }
}

/// Represents game results.
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum GameResult {
    Wins(Color, WinType),
    Draw(DrawType),
}

impl fmt::Display for GameResult {
    /// Represents the game result as a string (1-0 if white wins, 0-1 if black wins, or 1/2-1/2 in the case of a draw).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Wins(c, _) =>
                    if c.is_white() {
                        "1-0"
                    } else {
                        "0-1"
                    },
                Self::Draw(_) => "1/2-1/2",
            }
        )
    }
}

/// Represents types of wins.
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum WinType {
    Checkmate,
    /// Currently, a loss by timeout is also considered a resignation.
    Resignation,
}

/// Represents types of draws.
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum DrawType {
    FivefoldRepetition,
    SeventyFiveMoveRule,
    /// Represents a stalemate, with the tuple value being the side in stalemate.
    Stalemate(Color),
    InsufficientMaterial,
    /// Currently, a claimed draw and a draw by timeout vs. insufficient checkmating material are also considered a draw by agreement.
    Agreement,
}

/// Represents a side/color.
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// Checks if the color is white.
    pub fn is_white(&self) -> bool {
        matches!(self, Self::White)
    }

    /// Checks if the color is black.
    pub fn is_black(&self) -> bool {
        matches!(self, Self::Black)
    }
}

impl TryFrom<&str> for Color {
    type Error = InvalidColorCharacterError;

    /// Attempts to convert a color character in a string slice to a `Color` ("w" is white, and "b" is black).
    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "w" => Ok(Self::White),
            "b" => Ok(Self::Black),
            _ => Err(InvalidColorCharacterError(string.to_string())),
        }
    }
}

impl From<Color> for char {
    /// Converts a `Color` to a color character (white is 'w', and black is 'b').
    fn from(c: Color) -> char {
        match c {
            Color::White => 'w',
            Color::Black => 'b',
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

/// Represents types of special moves (castling/promotion/en passant).
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum SpecialMoveType {
    CastlingKingside,
    CastlingQueenside,
    /// Represents a promotion, with the tuple value being the type of piece that the pawn promotes to.
    Promotion(PieceType),
    EnPassant,
    Unclear,
}

#[cfg(test)]
mod test;

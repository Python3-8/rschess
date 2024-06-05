//! A Rust chess library with the aim to be as feature-rich as possible
//! # Examples
//! ```
//! use rschess::{Board, Color, Fen, Move, GameResult, WinType};
//!
//! let mut board = Board::default();
//! assert_eq!(board.to_fen(), Fen::try_from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap());
//! assert!(board.is_ongoing()); // the game is ongoing
//! assert!(board.side_to_move().is_white()); // white's turn to move
//! board.make_move_uci("e2e4").unwrap(); // plays e2 to e4, i.e. 1. e4
//! assert!(board.side_to_move().is_black()); // black's turn to move
//! board.make_move_san("e5").unwrap(); // plays 1... e5
//! assert!(board.is_legal(board.san_to_move("f4").unwrap())); // confirms that 2. f4 is legal
//! assert!(board.is_legal(Move::from_uci("d2d4").unwrap())); // confirms that d2 to d4, i.e. 2. d4 is legal
//! assert!(board.san_to_move("Ne4").is_err()); // confirms that 2. Ne4 is invalid in this position
//! assert!(!board.is_legal(Move::from_uci("e1g1").unwrap())); // confirms that e1 to g1, i.e. 2. O-O is invalid
//! assert_eq!(board.halfmove_clock(), 0); // confirms that the halfmove clock has been reset (since the last move was a pawn move)
//! board.make_move_san("Nf3").unwrap(); // plays 2. Nf3
//! assert_eq!(board.halfmove_clock(), 1); // confirms that the halfmove clock has incremented (since 2. Nf3 was not a pawn push or capture)
//! board.make_move_san("f6").unwrap(); // plays 2... f6
//! board.make_move_san("Nxe5").unwrap(); // plays 3. Nxe5
//! assert_eq!(board.halfmove_clock(), 0); // confirms that the halfmove clock has been reset (since the last move was a capture)
//! board.make_move_san("fxe5").unwrap(); // plays 3... fxe5
//! board.make_move_san("Qh5+").unwrap(); // plays 4. Qh5+
//! assert!(board.is_check()); // confirms that a side is in check
//! assert_eq!(board.checked_side(), Some(Color::Black)); // confirms that black is the side in check
//! assert_eq!(board.gen_legal_moves().len(), 2); // confirms that there are only two legal moves (4... g6 and 4... Ke7)
//! board.make_move_uci("e8e7").unwrap(); // plays e8 to e7, i.e. 4... Ke7
//! assert_eq!(board.halfmove_clock(), 2); // confirms that the halfmove clock has incremented twice (since two halfmoves have been played without a pawn push or capture)
//! board.make_move_uci("h5e5").unwrap(); // plays h5 to e5, i.e. 5. Qxe5+
//! assert_eq!(board.halfmove_clock(), 0); // confirms that the halfmove clock has been reset (since the last move was a capture)
//! board.make_move_san("Kf7").unwrap(); // plays 5... Kf7
//! board.make_move_san("Bc4+").unwrap(); // plays 6. Bc4+
//! board.make_move_san("Kg6").unwrap(); // plays 6... Kg6
//! board.make_move_san("Qf5+").unwrap(); // plays 7. Qf5+
//! assert_eq!(board.gen_legal_moves().len(), 1); // confirms that there is only one legal move
//! board.make_move_san("Kh6").unwrap(); // plays 7... Kh6
//! board.make_move_san("d4+").unwrap(); // plays 8. d4+ (discovered check by the bishop on c1)
//! assert!(board.is_check()); // confirms that a side is in check
//! board.make_move_san("g5").unwrap(); // plays 8... g5
//! board.make_move_san("h4").unwrap(); // plays 9. h4
//! board.make_move_san("Bg7").unwrap(); // plays 9... Bg7
//! board.make_move_san("hxg5#").unwrap(); // plays 10. hxg5#
//! assert!(board.is_game_over()); // confirms that the game is over
//! assert!(board.is_checkmate()); // confirms that a side has been checkmated
//! assert_eq!(board.game_result(), Some(GameResult::Wins(Color::White, WinType::Checkmate))); // confirms that white has won
//! assert_eq!(board.fullmove_number(), 10); // confirms that the current fullmove number is 10
//! assert_eq!(board.gen_legal_moves().len(), 0); // confirms that there are no legal moves because the game is over
//! ```
mod board;
mod fen;
mod helpers;
mod pgn;
mod position;

pub use board::Board;
pub use fen::Fen;
pub use pgn::Pgn;
pub use position::Position;
use std::{fmt, ops::Not};

/// Represents a piece in the format (_piece type_, _color_).
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
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
    type Error = String;

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
    type Error = String;

    /// Attempts to convert a piece character to a `PieceType`.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        if !value.is_ascii_alphanumeric() {
            return Err(format!("Invalid piece character: '{value}' is not ASCII alphanumeric"));
        }
        Ok(match value.to_ascii_lowercase() {
            'k' => Self::K,
            'q' => Self::Q,
            'b' => Self::B,
            'n' => Self::N,
            'r' => Self::R,
            'p' => Self::P,
            _ => return Err(format!("Invalid piece character: '{value}' does not correspond to any chess piece")),
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

/// The structure for a chess move, in the format (_source square_, _destination square_, _castling/promotion/en passant_)
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
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
    pub fn from_uci(uci: &str) -> Result<Self, String> {
        let uci_len = uci.len();
        if ![4, 5].contains(&uci_len) {
            return Err(format!("Invalid UCI: Expected string to be 4 or 5 characters long, got {uci_len}"));
        }
        let from_square = (uci.chars().next().unwrap(), uci.chars().nth(1).unwrap());
        let to_square = (uci.chars().nth(2).unwrap(), uci.chars().nth(3).unwrap());
        let promotion = uci.chars().nth(4);
        if !(('a'..='h').contains(&from_square.0) && ('1'..='8').contains(&from_square.1)) {
            return Err(format!("Invalid UCI: '{}{}' is not a valid square name", from_square.0, from_square.1));
        }
        if !(('a'..='h').contains(&to_square.0) && ('1'..='8').contains(&to_square.1)) {
            return Err(format!("Invalid UCI: '{}{}' is not a valid square name", to_square.0, to_square.1));
        }
        let (src, dest) = (helpers::sq_to_idx(from_square.0, from_square.1), helpers::sq_to_idx(to_square.0, to_square.1));
        let promotion = match promotion {
            Some(p) => Some(PieceType::try_from(p)?),
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

/// Represents game results.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
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
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum WinType {
    Checkmate,
    /// Currently, a loss by timeout is also considered a resignation.
    Resignation,
}

/// Represents types of draws.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum DrawType {
    FivefoldRepetition,
    SeventyFiveMoveRule,
    Stalemate(Color),
    InsufficientMaterial,
    /// Currently, a claimed draw is also considered a draw by agreement.
    Agreement,
}

/// Represents a side/color.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
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
    type Error = String;

    /// Attempts to convert a color character in a string slice to a `Color` ("w" is white, and "b" is black).
    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "w" => Ok(Self::White),
            "b" => Ok(Self::Black),
            _ => Err(format!("Color character must be 'w' or 'b', got '{string}'")),
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
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum SpecialMoveType {
    CastlingKingside,
    CastlingQueenside,
    Promotion(PieceType),
    EnPassant,
    Unclear,
}

/// The error type used to convey the illegality of a move
#[derive(Debug)]
pub struct IllegalMoveError;

impl fmt::Display for IllegalMoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Illegal move: This move is illegal")
    }
}

impl std::error::Error for IllegalMoveError {}

#[cfg(test)]
mod test;

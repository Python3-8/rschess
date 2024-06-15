//! Contains all rschess error types.

use super::Move;
use thiserror::Error;

/// Conveys that the given FEN is invalid.
#[derive(Error, Debug)]
pub enum InvalidFenError {
    #[error("Invalid FEN: expected six fields separated by a space")]
    SixFields,
    #[error("Invalid FEN board data: {0}")]
    BoardData(String),
    #[error("Invalid FEN: active color must be 'w' or 'b'")]
    ActiveColor,
    #[error("Invalid FEN castling rights: {0}")]
    CastlingRights(String),
    #[error("Invalid FEN en passant target square: this field must be '-' or a valid square name on the 3rd or 6th rank")]
    EnPassantTargetSquare,
    #[error("Invalid FEN halfmove clock: halfmove clock must be in the range 0..=150")]
    HalfmoveClock,
    #[error("Invalid FEN fullmove number: fullmove number must be in the range 1..")]
    FullmoveNumber,
}

/// Conveys that the given piece character is invalid.
#[derive(Error, Debug)]
#[error("Invalid piece character: '{0}'; a valid piece character must be /[KkQqRrBbNnPp]/gi")]
pub struct InvalidPieceCharacterError(pub char);

/// Conveys that the given UCI text is invalid.
#[derive(Error, Debug)]
pub enum InvalidUciError {
    #[error("Invalid UCI: expected UCI to be 4 to 5 characters long")]
    Length,
    #[error("Invalid UCI: '{0}{1}' is not a valid square name")]
    InvalidSquareName(char, char),
    #[error("Invalid UCI: '{0}' is not a valid piece character for promotion")]
    InvalidPieceType(char),
}

/// Conveys that the given color character is invalid.
#[derive(Error, Debug)]
#[error("Invalid color character: '{0}', a valid color character must be 'w' or 'b'")]
pub struct InvalidColorCharacterError(pub String);

/// Conveys that the given move is illegal.
#[derive(Error, Debug)]
#[error("Illegal move: {0}")]
pub struct IllegalMoveError(pub Move);

/// Conveys that the given UCI move is either invalid or illegal.
#[derive(Error, Debug)]
pub enum InvalidUciMoveError {
    #[error("Invalid UCI move: '{0}' is not valid UCI")]
    InvalidUci(String),
    #[error("Invalid UCI move: '{0}' is illegal in this position")]
    IllegalMove(String),
}

/// Conveys that the given SAN move is either invalid or illegal.
#[derive(Error, Debug)]
#[error("Invalid SAN move: '{0}' is either invalid or illegal in this position")]
pub struct InvalidSanMoveError(pub String);

/// Conveys that the given square name is invalid.
#[derive(Error, Debug)]
#[error("Invalid square name: {0}{1}")]
pub struct InvalidSquareNameError(pub char, pub char);

/// Conveys that the given square index is invalid.
#[derive(Error, Debug)]
#[error("Invalid square index: {0}")]
pub struct InvalidSquareIndexError(pub usize);

/// Conveys that this action cannot be taken after the game is over.
#[derive(Error, Debug)]
pub enum GameOverError {
    #[error("Game over: a player cannot resign when the game is over")]
    Resignation,
    #[error("Game over: players cannot agree to a draw when the game is over")]
    AgreementDraw,
}

/// Conveys that the given PGN text is invalid.
#[derive(Error, Debug)]
pub enum InvalidPgnError {
    #[error("Invalid PGN: the elements are incorrectly organized, {0}")]
    OrderOfElements(String),
    #[error("Invalid PGN: move numbers cannot be less than 1, and successive move numbers must differ by 1")]
    InvalidMoveNumber,
    #[error("Invalid PGN: variations (and annotations) are not yet supported; all movetext must include only fullmoves and a halfmove is only allowed on the last move")]
    NoAnnotations,
    #[error("Invalid PGN: tag pairs must follow the Seven Tag Roster (https://en.wikipedia.org/wiki/Portable_Game_Notation#Seven_Tag_Roster)")]
    SevenTagRoster,
    #[error("Invalid PGN: {0}")]
    InvalidMove(InvalidSanMoveError),
    #[error("Invalid PGN: invalid result, {0}")]
    InvalidResult(String),
}

/// Conveys that the given RGB or hex color is invalid.
#[derive(Error, Debug)]
#[error("Invalid hex: '{0}' is not a valid hex color")]
pub struct InvalidHexError(pub String);

/// Conveys that the given piece set name is invalid.
#[derive(Error, Debug)]
pub enum InvalidPositionImagePropertiesError<'a> {
    #[error("Invalid position image properties: the size {0} must be at least 8 pixels")]
    InvalidSize(usize),
    #[error("Invalid position image properties: '{0}' is not a recognized piece set")]
    InvalidPieceSet(&'a str),
}

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
mod helpers;
mod pgn;
mod position;

pub use pgn::Pgn;
use position::Position;
use std::{fmt, ops::Not};

/// The structure for a chessboard/game
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Board {
    /// The position on the board
    position: Position,
    /// The number of halfmoves since the last pawn push or capture
    halfmove_clock: usize,
    /// The current fullmove number
    fullmove_number: usize,
    /// Whether or not the game is still in progress
    ongoing: bool,
    /// The list of positions that have occurred on the board
    position_history: Vec<Position>,
    /// The list of moves that have occurred on the board
    move_history: Vec<Move>,
    /// The FEN string representing the initial game state
    initial_fen: Fen,
    /// The side that has resigned (or lost by timeout)
    resigned_side: Option<Color>,
    /// Whether a draw has been made by agreement (or claimed)
    draw_agreed: bool,
}

impl Board {
    /// Constructs a `Board` from a `Fen` object.
    pub fn from_fen(fen: Fen) -> Self {
        let Fen {
            position,
            halfmove_clock,
            fullmove_number,
        } = fen.clone();
        let mut board = Self {
            position,
            halfmove_clock,
            fullmove_number,
            ongoing: halfmove_clock < 150,
            position_history: Vec::new(),
            move_history: Vec::new(),
            initial_fen: fen,
            resigned_side: None,
            draw_agreed: false,
        };
        board.check_game_over();
        board
    }

    /// Returns a `Fen` object representing the `Board`.
    pub fn to_fen(&self) -> Fen {
        Fen {
            position: self.position.clone(),
            halfmove_clock: self.halfmove_clock,
            fullmove_number: self.fullmove_number,
        }
    }

    /// Represents a `Move` in SAN, returning an error if the move is illegal.
    pub fn move_to_san(&self, move_: Move) -> Result<String, IllegalMoveError> {
        let move_ = helpers::as_legal(move_, &self.gen_legal_moves()).ok_or(IllegalMoveError)?;
        self.position.move_to_san(move_)
    }

    /// Constructs a `Move` from a SAN representation, returning an error if it is invalid or illegal.
    pub fn san_to_move(&self, san: &str) -> Result<Move, String> {
        match self.position.san_to_move(san) {
            Ok(m) => {
                if self.is_legal(m) {
                    Ok(m)
                } else {
                    Err(format!("Invalid SAN: this move '{san}' is illegal in this position"))
                }
            }
            e => e,
        }
    }

    /// Generates the legal moves in the position.
    pub fn gen_legal_moves(&self) -> Vec<Move> {
        if self.ongoing {
            self.position.gen_non_illegal_moves()
        } else {
            Vec::new()
        }
    }

    /// Checks whether a move is legal in the position.
    pub fn is_legal(&self, move_: Move) -> bool {
        helpers::as_legal(move_, &self.gen_legal_moves()).is_some()
    }

    /// Plays on the board the given move, returning an error if the move is illegal.
    pub fn make_move(&mut self, move_: Move) -> Result<(), IllegalMoveError> {
        let move_ = match helpers::as_legal(move_, &self.gen_legal_moves()) {
            Some(m) => m,
            _ => return Err(IllegalMoveError),
        };
        let mut halfmove_clock = self.halfmove_clock;
        let fullmove_number = self.fullmove_number + if self.position.side.is_black() { 1 } else { 0 };
        let Move(move_src, move_dest, ..) = move_;
        let (moved_piece, dest_occ) = (self.position.content[move_src], self.position.content[move_dest]);
        if matches!(moved_piece, Some(Piece(PieceType::P, _))) || dest_occ.is_some() {
            halfmove_clock = 0;
        } else {
            halfmove_clock += 1;
        }
        self.position_history.push(self.position.clone());
        self.position = self.position.make_move(move_).unwrap();
        self.move_history.push(move_);
        (self.halfmove_clock, self.fullmove_number) = (halfmove_clock, fullmove_number);
        self.check_game_over();
        Ok(())
    }

    /// Attempts to parse the UCI representation of a move and play it on the board, returning an error if the move is invalid or illegal.
    pub fn make_move_uci(&mut self, uci: &str) -> Result<(), String> {
        let move_ = Move::from_uci(uci)?;
        self.make_move(move_).map_err(|e| format!("{e}"))
    }

    /// Attempts to interpret the SAN representation of a move and play it on the board, returning an error if it is invalid or illegal.
    pub fn make_move_san(&mut self, san: &str) -> Result<(), String> {
        let move_ = self.san_to_move(san)?;
        self.make_move(move_).map_err(|e| format!("{e}"))
    }

    /// Updates the `ongoing` property of the `Board` if the game is over
    fn check_game_over(&mut self) {
        if self.is_fivefold_repetition() || self.is_seventy_five_move_rule() || self.is_stalemate() || self.is_insufficient_material() || self.is_checkmate() {
            self.ongoing = false;
        }
    }

    /// Checks whether the game is still ongoing.
    pub fn is_ongoing(&self) -> bool {
        self.ongoing
    }

    /// Checks whether the game is over.
    pub fn is_game_over(&self) -> bool {
        !self.ongoing
    }

    /// Returns an optional game result (`None` if the game is ongoing).
    pub fn game_result(&self) -> Option<GameResult> {
        if self.ongoing {
            None
        } else {
            Some(if self.draw_agreed {
                GameResult::Draw(DrawType::Agreement)
            } else if let Some(s) = self.resigned_side {
                GameResult::Wins(!s, WinType::Resignation)
            } else {
                match self.checkmated_side() {
                    Some(Color::Black) => GameResult::Wins(Color::White, WinType::Checkmate),
                    Some(Color::White) => GameResult::Wins(Color::Black, WinType::Checkmate),
                    None => {
                        if let Some(s) = self.stalemated_side() {
                            GameResult::Draw(DrawType::Stalemate(s))
                        } else if self.is_fivefold_repetition() {
                            GameResult::Draw(DrawType::FivefoldRepetition)
                        } else if self.is_seventy_five_move_rule() {
                            GameResult::Draw(DrawType::SeventyFiveMoveRule)
                        } else if self.is_insufficient_material() {
                            GameResult::Draw(DrawType::InsufficientMaterial)
                        } else {
                            panic!("the universe is malfunctioning")
                        }
                    }
                }
            })
        }
    }

    /// Returns the number of halfmoves since the last pawn push or capture.
    pub fn halfmove_clock(&self) -> usize {
        self.halfmove_clock
    }

    /// Returns the fullmove number.
    pub fn fullmove_number(&self) -> usize {
        self.fullmove_number
    }

    /// Checks whether a threefold repetition of the position has occurred.
    pub fn is_threefold_repetition(&self) -> bool {
        self.position_history.iter().fold(0, |acc, pos| if pos == &self.position { acc + 1 } else { acc }) == 3
    }

    /// Checks whether a fivefold repetition of the position has occurred.
    pub fn is_fivefold_repetition(&self) -> bool {
        self.position_history.iter().fold(0, |acc, pos| if pos == &self.position { acc + 1 } else { acc }) == 5
    }

    /// Checks whether a draw can be claimed by the fifty-move rule.
    pub fn is_fifty_move_rule(&self) -> bool {
        self.halfmove_clock == 100
    }

    /// Checks whether the game is drawn by the seventy-five-move rule.
    pub fn is_seventy_five_move_rule(&self) -> bool {
        self.halfmove_clock == 150
    }

    /// Checks whether the game is drawn by stalemate. Use [`Board::stalemated_side`] to know which side is in stalemate.
    pub fn is_stalemate(&self) -> bool {
        self.position.is_stalemate()
    }

    /// Checks whether the game is drawn by insufficient material.
    ///
    /// rschess defines insufficient material as any of the following scenarios:
    /// * King and knight vs. king
    /// * King and zero or more bishops vs. king and zero or more bishops where all the bishops are on the same color complex
    pub fn is_insufficient_material(&self) -> bool {
        self.position.is_insufficient_material()
    }

    /// Checks whether there is sufficient checkmating material on the board.
    pub fn is_sufficient_material(&self) -> bool {
        !self.is_insufficient_material()
    }

    /// Checks whether any side is in check (a checkmate is also considered a check). Use [`Board::checked_side`] to know which side is in check.
    pub fn is_check(&self) -> bool {
        self.position.is_check()
    }

    /// Checks whether any side is in checkmate. Use [`Board::checkmated_side`] to know which side is in checkmate.
    pub fn is_checkmate(&self) -> bool {
        self.position.is_checkmate()
    }

    /// Returns an optional `Color` representing the side in stalemate (`None` if neither side is in stalemate).
    pub fn stalemated_side(&self) -> Option<Color> {
        self.position.stalemated_side()
    }

    /// Returns an optional `Color` representing the side in check (`None` if neither side is in check).
    pub fn checked_side(&self) -> Option<Color> {
        self.position.checked_side()
    }

    /// Returns an optional `Color` representing the side in checkmate (`None` if neither side is in checkmate).
    pub fn checkmated_side(&self) -> Option<Color> {
        self.position.checkmated_side()
    }

    /// Pretty-prints the position to a string, from the perspective of the side `perspective`.
    pub fn pretty_print(&self, perspective: Color) -> String {
        self.position.pretty_print(perspective)
    }

    /// Returns which side's turn it is to move.
    pub fn side_to_move(&self) -> Color {
        self.position.side
    }

    /// Returns the occupant of a square, or an error if the square name is invalid.
    pub fn occupant_of_square(&self, file: char, rank: char) -> Result<Option<Piece>, String> {
        if !('a'..'h').contains(&file) {
            return Err(format!("Invalid file name: {file}"));
        }
        if !('1'..'8').contains(&rank) {
            return Err(format!("Invalid rank: {rank}"));
        }
        Ok(self.position.content[helpers::sq_to_idx(file, rank)])
    }

    /// Resigns the game for a certain side, if the game is ongoing. Currently, this function should also be used to represent a loss by timeout.
    pub fn resign(&mut self, side: Color) -> Result<(), String> {
        if !self.ongoing {
            return Err("A player cannot resign when the game is already over".to_owned());
        }
        self.ongoing = false;
        self.resigned_side = Some(side);
        Ok(())
    }

    /// Makes a draw by agreement, if the game is ongoing. Currently, this function should also be used to represent a draw claim.
    pub fn agree_draw(&mut self) -> Result<(), String> {
        if !self.ongoing {
            return Err("Players cannot agree to a draw when the game is already over".to_owned());
        }
        self.ongoing = false;
        self.draw_agreed = true;
        Ok(())
    }

    /// Returns an optional `Color` representing the side that has resigned (`None` if neither side has resigned).
    pub fn resigned_side(&self) -> Option<Color> {
        self.resigned_side
    }

    /// Checks whether a draw has been agreed upon.
    pub fn draw_agreed(&self) -> bool {
        self.draw_agreed
    }

    /// Returns the initial FEN of the game.
    pub fn initial_fen(&self) -> &Fen {
        &self.initial_fen
    }

    /// Generates the SAN movetext of the game thus far (excluding the game result)
    pub fn gen_movetext(&self) -> String {
        let mut movetext = String::new();
        let initial_side = self.initial_fen.position.side;
        let initial_fullmove_number: usize = self.initial_fen.fullmove_number;
        let mut current_side = initial_side;
        let mut current_fullmove_number = initial_fullmove_number;
        for (movei, &move_) in self.move_history.iter().enumerate() {
            let pos = &self.position_history[movei];
            let san = pos.move_to_san(move_).unwrap();
            if current_side.is_black() {
                movetext.push_str(&format!("{}{san} ", if movei == 0 { format!("{current_fullmove_number}... ") } else { String::new() }));
                current_fullmove_number += 1;
            } else {
                movetext.push_str(&format!("{current_fullmove_number}. {san} "))
            }
            current_side = !current_side;
        }
        movetext.trim().to_owned()
    }
}

impl Default for Board {
    /// Constructs a `Board` with the starting position for a chess game.
    fn default() -> Self {
        Self::from_fen(Fen::try_from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap())
    }
}

/// Represents FEN (Forsyth-Edwards Notation).
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Fen {
    position: Position,
    halfmove_clock: usize,
    fullmove_number: usize,
}

impl TryFrom<&str> for Fen {
    type Error = String;

    /// Attempts to construct a `Fen` object from a string slice, returning an error if it is invalid.
    /// **Shredder-FEN is NOT supported**.
    fn try_from(fen: &str) -> Result<Self, Self::Error> {
        let mut content = [None; 64];
        let fields: Vec<_> = fen.split(' ').collect();
        let nfields = fields.len();
        if nfields != 6 {
            return Err(format!("Invalid FEN: expected six space-separated FEN fields, got {nfields}"));
        }
        let ranks: Vec<_> = fields[0].split('/').collect();
        let nranks = ranks.len();
        if nranks != 8 {
            return Err(format!("Invalid FEN: expected eight ranks of pieces separated by forward-slashes, got {nranks}"));
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
                    return Err(format!("Invalid FEN: rank {rankn} cannot have pieces beyond the h file (8 squares already occupied)"));
                }
                if piece_char.is_ascii_digit() {
                    let empty_space = piece_char.to_digit(10).unwrap() as usize;
                    if !(1..=8).contains(&empty_space) {
                        return Err(format!("Invalid FEN: {empty_space} is not a valid character for board data, digits must be in the range 1..=8"));
                    }
                    if empty_space > 8 - rank_filled {
                        return Err(format!(
                            "Invalid FEN: rank {rankn} only has 8 squares, {rank_filled} of which is/are occupied. {empty_space} more squares of empty space cannot be accomodated"
                        ));
                    }
                    rank_filled += empty_space;
                    ptr = ptr.saturating_sub(empty_space);
                } else {
                    content[ptr] = match piece_char.try_into() {
                        Ok(piece) => {
                            match piece {
                                Piece(PieceType::K, Color::White) => {
                                    if wk_seen {
                                        return Err("Invalid FEN: white cannot have more than one king".to_owned());
                                    }
                                    wk_seen = true;
                                    wk_pos = ptr;
                                }
                                Piece(PieceType::K, Color::Black) => {
                                    if bk_seen {
                                        return Err("Invalid FEN: black cannot have more than one king".to_owned());
                                    }
                                    bk_seen = true;
                                    bk_pos = ptr;
                                }
                                Piece(PieceType::P, _) => {
                                    if !(8..56).contains(&ptr) {
                                        return Err("Invalid FEN: there cannot be pawns on the 1st and 8th ranks".to_owned());
                                    }
                                }
                                _ => (),
                            }
                            Some(piece)
                        }
                        Err(e) => return Err(format!("Invalid FEN: {e}")),
                    };
                    rank_filled += 1;
                    ptr = ptr.saturating_sub(1);
                }
            }
            if rank_filled != 8 {
                return Err(format!("Invalid FEN: rank {rankn} does not have data for 8 squares"));
            }
            rankn -= 1;
        }
        if !(wk_seen && bk_seen) {
            return Err("Invalid FEN: a valid chess position must have one white king and one black king".to_owned());
        }
        let turn = fields[1];
        let side = match Color::try_from(turn) {
            Ok(c) => c,
            _ => return Err(format!("Invalid FEN: Expected second field (side to move) to be 'w' or 'b', got '{turn}'")),
        };
        if helpers::king_capture_pseudolegal(&content, side) {
            return Err("Invalid FEN: When one side is in check, it cannot be the other side's turn to move".to_owned());
        }
        let castling = fields[2];
        let len_castling = castling.len();
        if !((1..=4).contains(&len_castling)) {
            return Err(format!(
                "Invalid FEN: Expected third field (castling rights) to be 1 to 4 characters long, got {len_castling} characters"
            ));
        }
        let mut castling_rights_old = [false; 4];
        if castling != "-" {
            for ch in castling.chars() {
                match ch {
                    'K' => {
                        if wk_pos > 6 {
                            return Err("Invalid FEN: White king must be from a1 to g1 to have kingside castling rights".to_owned());
                        }
                        if castling_rights_old[0] {
                            return Err("Invalid FEN: Found more than one occurrence of 'K' in third field (castling rights)".to_owned());
                        }
                        castling_rights_old[0] = true;
                    }
                    'Q' => {
                        if !(1..=7).contains(&wk_pos) {
                            return Err("Invalid FEN: White king must be from b1 to h1 to have queenside castling rights".to_owned());
                        }
                        if castling_rights_old[1] {
                            return Err("Invalid FEN: Found more than one occurrence of 'Q' in third field (castling rights)".to_owned());
                        }
                        castling_rights_old[1] = true;
                    }
                    'k' => {
                        if !(56..=62).contains(&bk_pos) {
                            return Err("Invalid FEN: Black king must be from a8 to g8 to have kingside castling rights".to_owned());
                        }
                        if castling_rights_old[2] {
                            return Err("Invalid FEN: Found more than one occurrence of 'k' in third field (castling rights)".to_owned());
                        }
                        castling_rights_old[2] = true;
                    }
                    'q' => {
                        if !(57..=63).contains(&bk_pos) {
                            return Err("Invalid FEN: Black king must be from b8 to h8 to have queenside castling rights".to_owned());
                        }
                        if castling_rights_old[3] {
                            return Err("Invalid FEN: Found more than one occurrence of 'q' in third field (castling rights)".to_owned());
                        }
                        castling_rights_old[3] = true;
                    }
                    _ => return Err(format!("Invalid FEN: Expected third field (castling rights) to contain '-' or a subset of 'KQkq', found '{ch}'")),
                }
            }
        }
        let count_rooks = |rng, color| helpers::count_piece(rng, Piece(PieceType::R, color), &content);
        if castling_rights_old[0] && count_rooks(wk_pos + 1..8, Color::White) != 1 {
            return Err("Invalid FEN: White must have exactly one king's rook to have kingside castling rights".to_owned());
        }
        if castling_rights_old[1] && count_rooks(0..wk_pos, Color::White) != 1 {
            return Err("Invalid FEN: White must have exactly one queen's rook to have queenside castling rights".to_owned());
        }
        if castling_rights_old[2] && count_rooks(bk_pos + 1..64, Color::Black) != 1 {
            return Err("Invalid FEN: Black must have exactly one king's rook to have kingside castling rights".to_owned());
        }
        if castling_rights_old[3] && count_rooks(56..bk_pos, Color::Black) != 1 {
            return Err("Invalid FEN: Black must have exactly one queen's rook to have queenside castling rights".to_owned());
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
            return Err(format!(
                "Invalid FEN: Expected fourth field (en passant target square) to be 1 to 2 characters long, got {len_ep} characters"
            ));
        }
        let mut ep_target = None;
        if ep != "-" {
            let err = Err(format!(
                "Invalid FEN: Expected fourth field (en passant target square) to be '-' or a valid en passant target square name, '{ep}' is not a valid en passant target square name"
            ));
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
        let halfmove_clock: usize = halfmoves
            .parse()
            .map_err(|_| format!("Invalid FEN: Expected fifth field (halfmove clock) to be a whole number, got '{halfmoves}'"))?;
        if halfmove_clock > 150 {
            return Err(format!(
                "Invalid FEN: Fifth field (halfmove clock) cannot contain a value greater than 150 (the seventy-five-move rule forces a draw when this reaches 150), got {halfmove_clock}"
            ));
        }
        let fullmoves = fields[5];
        let fullmove_number: usize = fullmoves
            .parse()
            .map_err(|_| format!("Invalid FEN: Expected sixth field (fullmove number) to be a natural number, got '{fullmoves}'"))?;
        if fullmove_number < 1 {
            return Err(format!(
                "Invalid FEN: Sixth field (fullmove number) cannot contain a value less than 1 (it starts at 1 and increments after Black's move), got {fullmove_number}"
            ));
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

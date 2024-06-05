use super::{helpers, Color, DrawType, Fen, GameResult, IllegalMoveError, Move, Piece, PieceType, Position, WinType};

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
        let (position, halfmove_clock, fullmove_number) = (fen.position().clone(), fen.halfmove_clock(), fen.fullmove_number());
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
        let initial_side = self.initial_fen.position().side;
        let initial_fullmove_number: usize = self.initial_fen.fullmove_number();
        let mut current_side = initial_side;
        let mut current_fullmove_number = initial_fullmove_number;
        for (movei, &move_) in self.move_history.iter().enumerate() {
            let pos = &self.position_history[movei];
            let san = pos.move_to_san(move_).unwrap();
            if current_side.is_black() {
                movetext.push_str(&format!("{}{san} ", if movei == 0 { format!("{current_fullmove_number}... ") } else { String::new() }));
                current_fullmove_number += 1;
            } else {
                movetext.push_str(&format!("{current_fullmove_number}. {san} "))
            }
            current_side = !current_side;
        }
        movetext.trim().to_owned()
    }

    /// Returns the current `Position` on the board.
    pub fn position(&self) -> &Position {
        &self.position
    }
}

impl Default for Board {
    /// Constructs a `Board` with the starting position for a chess game.
    fn default() -> Self {
        Self::from_fen(Fen::try_from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap())
    }
}

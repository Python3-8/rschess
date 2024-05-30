//! rschess is yet another chess library for Rust, with the aim to be as feature-rich as possible. It is still IN DEVELOPMENT, and NOT FIT FOR USE.

mod helpers;

/// The structure for a chess position
#[derive(Eq, PartialEq, Clone, Debug)]
struct Position {
    /// The board content; each square is represented by a number 0..64 where a1 is 0 and h8 is 63
    content: [Occupant; 64],
    /// The side to move; white is `true` and black is `false`
    side: bool,
    /// The castling rights for both sides in the format [K, Q, k, q]
    castling_rights: [bool; 4],
    /// The index of the en passant target square, 0..64
    ep_target: Option<usize>,
}

impl Position {
    /// Generates pseudolegal moves in the position.
    fn gen_pseudolegal_moves(&self) -> Vec<Move> {
        let Self {
            content,
            castling_rights,
            ep_target,
            side,
        } = self;
        let mut pseudolegal_moves = Vec::new();
        for (i, sq) in content.into_iter().enumerate() {
            if let Occupant::Piece(piece) = sq {
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
                        possible_dests = possible_dests
                            .into_iter()
                            .filter(|&dest| match content[dest] {
                                Occupant::Piece(Piece(_, color)) => color != *side,
                                _ => true,
                            })
                            .collect();
                        pseudolegal_moves.extend(possible_dests.into_iter().map(|d| Move(i, d, None)));
                        let castling_rights_idx_offset = if *side { 0 } else { 2 };
                        let (oo_sq, ooo_sq) = if *side { (6, 2) } else { (62, 58) };
                        if castling_rights[castling_rights_idx_offset] && helpers::count_pieces(i + 1..=oo_sq, content) <= 1 {
                            pseudolegal_moves.push(Move(i, oo_sq, Some(PieceType::K)));
                        }
                        if castling_rights[castling_rights_idx_offset + 1] && helpers::count_pieces(ooo_sq..i, content) <= 1 {
                            pseudolegal_moves.push(Move(i, ooo_sq, Some(PieceType::K)));
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
                                    Occupant::Piece(Piece(_, color)) => color != *side,
                                    _ => true,
                                })
                                .map(|dest| Move(i, dest, None)),
                        )
                    }
                    PieceType::P => {
                        let mut possible_dests = Vec::new();
                        if *side {
                            if let Occupant::Empty = content[i + 8] {
                                possible_dests.push((i + 8, false));
                                if (8..16).contains(&i) && content[i + 16] == Occupant::Empty {
                                    possible_dests.push((i + 16, false))
                                }
                            }
                            if helpers::long_range_can_move(i, 7) {
                                if let Occupant::Piece(Piece(_, color)) = content[i + 7] {
                                    if !color {
                                        possible_dests.push((i + 7, false));
                                    }
                                } else if ep_target.is_some() && ep_target.unwrap() == i + 7 {
                                    possible_dests.push((i + 7, true));
                                }
                            }
                            if helpers::long_range_can_move(i, 9) {
                                if let Occupant::Piece(Piece(_, color)) = content[i + 9] {
                                    if !color {
                                        possible_dests.push((i + 9, false));
                                    }
                                } else if ep_target.is_some() && ep_target.unwrap() == i + 9 {
                                    possible_dests.push((i + 9, true));
                                }
                            }
                        } else {
                            if let Occupant::Empty = content[i - 8] {
                                possible_dests.push((i - 8, false));
                                if (48..56).contains(&i) && content[i - 16] == Occupant::Empty {
                                    possible_dests.push((i - 16, false))
                                }
                            }
                            if helpers::long_range_can_move(i, -9) {
                                if let Occupant::Piece(Piece(_, color)) = content[i - 9] {
                                    if color {
                                        possible_dests.push((i - 9, false));
                                    }
                                } else if ep_target.is_some() && ep_target.unwrap() == i - 9 {
                                    possible_dests.push((i - 9, true));
                                }
                            }
                            if helpers::long_range_can_move(i, -7) {
                                if let Occupant::Piece(Piece(_, color)) = content[i - 7] {
                                    if color {
                                        possible_dests.push((i - 7, false));
                                    }
                                } else if ep_target.is_some() && ep_target.unwrap() == i - 7 {
                                    possible_dests.push((i - 7, true));
                                }
                            }
                        }
                        pseudolegal_moves.extend(
                            possible_dests
                                .into_iter()
                                .map(|(dest, ep)| {
                                    if (0..8).contains(&dest) || (56..64).contains(&dest) {
                                        [PieceType::Q, PieceType::R, PieceType::B, PieceType::N].into_iter().map(|p| Move(i, dest, Some(p))).collect()
                                    } else {
                                        vec![Move(i, dest, if ep { Some(PieceType::P) } else { None })]
                                    }
                                })
                                .flatten(),
                        );
                    }
                    long_range_type => pseudolegal_moves.append(&mut self.gen_long_range_piece_pseudolegal_moves(i, long_range_type)),
                }
            }
        }
        pseudolegal_moves
    }

    /// Generates pseudolegal moves for a long-range piece.
    fn gen_long_range_piece_pseudolegal_moves(&self, sq: usize, piece_type: PieceType) -> Vec<Move> {
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
                    match content[current_sq as usize] {
                        Occupant::Piece(Piece(_, color)) => {
                            if color == *side {
                                continue 'axis;
                            } else {
                                skip = true;
                            }
                        }
                        _ => (),
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
    pub fn controls_square(&self, sq: usize, side: bool) -> bool {
        let Self {
            mut content,
            castling_rights,
            ep_target,
            ..
        } = self.clone();
        content[sq] = Occupant::Piece(Piece(PieceType::P, !side));
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
}

/// The structure for a chessboard/game
#[derive(Debug)]
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
}

impl Board {
    /// Attempts to construct a `Board` from a standard FEN string, returning an error if the FEN is invalid.
    /// **Shredder-FEN is not supported.**
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let mut content = [Occupant::Empty; 64];
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
                                Piece(PieceType::K, true) => {
                                    if wk_seen {
                                        return Err("Invalid FEN: white cannot have more than one king".to_owned());
                                    }
                                    wk_seen = true;
                                    wk_pos = ptr;
                                }
                                Piece(PieceType::K, false) => {
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
                            Occupant::Piece(piece)
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
        let side;
        match turn {
            "w" => side = true,
            "b" => side = false,
            _ => return Err(format!("Invalid FEN: Expected second field (side to move) to be 'w' or 'b', got '{turn}'")),
        }
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
        let mut castling_rights = [false; 4];
        if castling != "-" {
            for ch in castling.chars() {
                match ch {
                    'K' => {
                        if wk_pos > 6 {
                            return Err("Invalid FEN: White king must be from a1 to g1 to have kingside castling rights".to_owned());
                        }
                        if castling_rights[0] {
                            return Err("Invalid FEN: Found more than one occurrence of 'K' in third field (castling rights)".to_owned());
                        }
                        castling_rights[0] = true;
                    }
                    'Q' => {
                        if !(1..=7).contains(&wk_pos) {
                            return Err("Invalid FEN: White king must be from b1 to h1 to have queenside castling rights".to_owned());
                        }
                        if castling_rights[1] {
                            return Err("Invalid FEN: Found more than one occurrence of 'Q' in third field (castling rights)".to_owned());
                        }
                        castling_rights[1] = true;
                    }
                    'k' => {
                        if !(56..=62).contains(&bk_pos) {
                            return Err("Invalid FEN: Black king must be from a8 to g8 to have kingside castling rights".to_owned());
                        }
                        if castling_rights[2] {
                            return Err("Invalid FEN: Found more than one occurrence of 'k' in third field (castling rights)".to_owned());
                        }
                        castling_rights[2] = true;
                    }
                    'q' => {
                        if !(57..=63).contains(&bk_pos) {
                            return Err("Invalid FEN: Black king must be from b8 to h8 to have queenside castling rights".to_owned());
                        }
                        if castling_rights[3] {
                            return Err("Invalid FEN: Found more than one occurrence of 'q' in third field (castling rights)".to_owned());
                        }
                        castling_rights[3] = true;
                    }
                    _ => return Err(format!("Invalid FEN: Expected third field (castling rights) to contain '-' or a subset of 'KQkq', found '{ch}'")),
                }
            }
        }
        let count_rooks = |rng, color| helpers::count_piece(rng, Piece(PieceType::R, color), &content);
        if castling_rights[0] && count_rooks(wk_pos + 1..8, true) != 1 {
            return Err("Invalid FEN: White must have exactly one king's rook to have kingside castling rights".to_owned());
        }
        if castling_rights[1] && count_rooks(0..wk_pos, true) != 1 {
            return Err("Invalid FEN: White must have exactly one queen's rook to have queenside castling rights".to_owned());
        }
        if castling_rights[2] && count_rooks(bk_pos + 1..64, false) != 1 {
            return Err("Invalid FEN: Black must have exactly one king's rook to have kingside castling rights".to_owned());
        }
        if castling_rights[3] && count_rooks(56..bk_pos, false) != 1 {
            return Err("Invalid FEN: Black must have exactly one queen's rook to have queenside castling rights".to_owned());
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
            position: position.clone(),
            halfmove_clock,
            fullmove_number,
            ongoing: halfmove_clock < 150,
            position_history: vec![position],
            move_history: Vec::new(),
        })
    }

    /// Returns the representation of the board state in standard FEN.
    pub fn to_fen(&self) -> String {
        let Position {
            content,
            side,
            castling_rights,
            ep_target,
        } = self.position;
        let mut rankstrs = Vec::new();
        for rank in content.chunks(8).rev() {
            let mut rankstr = String::new();
            let mut empty = 0;
            for sq in rank {
                match sq {
                    Occupant::Piece(p) => {
                        if empty > 0 {
                            rankstr.push(char::from_digit(empty, 10).unwrap());
                            empty = 0;
                        }
                        rankstr.push((*p).into());
                    }
                    Occupant::Empty => {
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
        let active_color = if side { "w".to_owned() } else { "b".to_owned() };
        let mut castling_availability = String::new();
        if castling_rights[0] {
            castling_availability.push('K');
        }
        if castling_rights[1] {
            castling_availability.push('Q');
        }
        if castling_rights[2] {
            castling_availability.push('k');
        }
        if castling_rights[3] {
            castling_availability.push('q');
        }
        if castling_availability.is_empty() {
            castling_availability.push('-');
        }
        let en_passant_target_square;
        if let Some(target) = ep_target {
            let (f, r) = helpers::idx_to_sq(target);
            en_passant_target_square = [f.to_string(), r.to_string()].join("");
        } else {
            en_passant_target_square = "-".to_owned();
        }
        [
            board_data,
            active_color,
            castling_availability,
            en_passant_target_square,
            self.halfmove_clock.to_string(),
            self.fullmove_number.to_string(),
        ]
        .join(" ")
    }

    /// Generates the legal moves in the position.
    pub fn gen_legal_moves(&self) -> Vec<Move> {
        if self.ongoing {
            let Position { content, side, .. } = self.position;
            self.position
                .gen_pseudolegal_moves()
                .into_iter()
                .filter(|move_| {
                    if let Move(src, dest, Some(PieceType::K)) = move_ {
                        for sq in *src..=*dest {
                            if self.position.controls_square(sq, !side) {
                                return false;
                            }
                        }
                        return true;
                    }
                    !helpers::king_capture_pseudolegal(&helpers::change_content(&content, move_), !side)
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Plays on the board the given move, returning an error if the move is illegal.
    pub fn make_move(&mut self, move_: Move) -> Result<(), ()> {
        let legal_moves = self.gen_legal_moves();
        if !legal_moves.contains(&move_) {
            return Err(());
        }
        let castling_rights_idx_offset = if self.position.side { 0 } else { 2 };
        let side = self.position.side;
        let mut castling_rights = self.position.castling_rights.clone();
        let mut ep_target = None;
        let mut halfmove_clock = self.halfmove_clock;
        let fullmove_number = self.fullmove_number + if side { 0 } else { 1 };
        let (move_src, moved_piece) = (move_.0, self.position.content[move_.0]);
        let (move_dest, dest_occ) = (move_.1, self.position.content[move_.1]);
        if let Occupant::Piece(Piece(piece_type, _)) = dest_occ {
            halfmove_clock = 0;
            if piece_type == PieceType::R {
                // TODO remove enemy side's appropriate castling rights
            }
        } else {
            halfmove_clock += 1;
        }
        match moved_piece {
            Occupant::Piece(Piece(PieceType::K, _)) => (castling_rights[castling_rights_idx_offset], castling_rights[castling_rights_idx_offset + 1]) = (false, false),
            Occupant::Piece(Piece(PieceType::P, _)) => {
                halfmove_clock = 0;
                if (std::cmp::max(move_src, move_dest) - std::cmp::min(move_src, move_dest)) == 16 {
                    ep_target = Some(if side { move_src + 8 } else { move_src - 8 });
                }
            }
            Occupant::Piece(Piece(PieceType::R, _)) => {
                // TODO remove appropriate castling rights
            }
            _ => (),
        }
        let side = !self.position.side;
        let new_content = helpers::change_content(&self.position.content, &move_);
        let new_position = Position {
            content: new_content,
            side: !side,
            castling_rights,
            ep_target,
        };
        self.position_history.push(self.position.clone());
        self.position = new_position;
        self.move_history.push(move_);
        (self.halfmove_clock, self.fullmove_number) = (halfmove_clock, fullmove_number);
        if self.is_fivefold_repetition() || self.is_seventy_five_move_rule() || self.is_stalemate() || self.is_insufficient_material() || self.is_checkmate() {
            self.ongoing = false;
        }
        Ok(())
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
            Some(match self.checkmated_side() {
                Some(false) => GameResult::WhiteWins,
                Some(true) => GameResult::BlackWins,
                None => match self.stalemated_side() {
                    Some(s) => GameResult::Draw(DrawType::Stalemate(s)),
                    None => {
                        if self.is_fivefold_repetition() {
                            GameResult::Draw(DrawType::FivefoldRepetition)
                        } else if self.is_seventy_five_move_rule() {
                            GameResult::Draw(DrawType::SeventyFiveMoveRule)
                        } else if self.is_insufficient_material() {
                            GameResult::Draw(DrawType::InsufficientMaterial)
                        } else {
                            panic!("the universe is malfunctioning")
                        }
                    }
                },
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

    /// Checks whether the game is drawn by stalemate. Use `Board::stalemated_side` to know which side is in stalemate.
    pub fn is_stalemate(&self) -> bool {
        !self.is_check() && self.gen_legal_moves().is_empty()
    }

    /// Checks whether the game is drawn by insufficient material.
    pub fn is_insufficient_material(&self) -> bool {
        todo!()
    }

    /// Checks whether any side is in check (a checkmate is also considered a check). Use `Board::checked_side` to know which side is in check.
    pub fn is_check(&self) -> bool {
        self.checked_side().is_some()
    }

    /// Checks whether any side is in checkmate. Use `Board::checkmated_side` to know which side is in checkmate.
    pub fn is_checkmate(&self) -> bool {
        self.is_check() && self.gen_legal_moves().is_empty()
    }

    /// Returns an optional boolean representing the side in stalemate (`None` if neither side is in stalemate).
    pub fn stalemated_side(&self) -> Option<bool> {
        if self.is_stalemate() {
            Some(self.position.side)
        } else {
            None
        }
    }

    /// Returns an optional boolean representing the side in check (`None` if neither side is in check).
    pub fn checked_side(&self) -> Option<bool> {
        if helpers::king_capture_pseudolegal(&self.position.content, false) {
            Some(true)
        } else if helpers::king_capture_pseudolegal(&self.position.content, true) {
            Some(false)
        } else {
            None
        }
    }

    /// Returns an optional boolean representing the side in checkmate (`None` if neither side is in checkmate).
    pub fn checkmated_side(&self) -> Option<bool> {
        if self.is_checkmate() {
            Some(self.position.side)
        } else {
            None
        }
    }
}

impl Default for Board {
    /// Constructs a `Board` with the starting position for a chess game.
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
struct Piece(PieceType, bool);

impl TryFrom<char> for Piece {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if !value.is_ascii_alphanumeric() {
            return Err(format!("Invalid piece character: '{value}' is not ASCII alphanumeric"));
        }
        let color = value.is_uppercase();
        Ok(Self(
            match value.to_ascii_lowercase() {
                'k' => PieceType::K,
                'q' => PieceType::Q,
                'b' => PieceType::B,
                'n' => PieceType::N,
                'r' => PieceType::R,
                'p' => PieceType::P,
                _ => return Err(format!("Invalid piece character: '{value}' does not correspond to any chess piece")),
            },
            color,
        ))
    }
}

impl From<Piece> for char {
    fn from(piece: Piece) -> char {
        let ch = match piece.0 {
            PieceType::K => 'k',
            PieceType::Q => 'q',
            PieceType::B => 'b',
            PieceType::N => 'n',
            PieceType::R => 'r',
            PieceType::P => 'p',
        };
        if piece.1 {
            ch.to_ascii_uppercase()
        } else {
            ch
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum PieceType {
    K,
    Q,
    B,
    N,
    R,
    P,
}

/// The structure for a chess move, in the format (<source square>, <destination square>, <castling/promotion/en passant>)
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Move(usize, usize, Option<PieceType>);

pub enum GameResult {
    WhiteWins,
    Draw(DrawType),
    BlackWins,
}

pub enum DrawType {
    FivefoldRepetition,
    SeventyFiveMoveRule,
    Stalemate(bool),
    InsufficientMaterial,
}

#[cfg(test)]
mod test;

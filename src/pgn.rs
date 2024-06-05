use super::{Board, Color, Fen, GameResult, InvalidPgnError};
use regex::Regex;
use std::{collections::HashMap, fmt};

const SEVEN_TAG_ROSTER: [&str; 7] = ["Event", "Site", "Date", "Round", "White", "Black", "Result"];

/// Represents PGN (Portable Game Notation).
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Pgn {
    tag_pairs: HashMap<String, String>,
    board: Board,
}

impl Pgn {
    /// Tokenizes PGN text.
    fn tokenize(text: &str) -> Vec<Token> {
        let tag_pair_regex = Regex::new(r#"\[(?<name>[A-Za-z]+)\s*"(?<value>((\\\\)|(\\")|[^"\\])*)"\]"#).unwrap();
        let fullmove_san_regex = Regex::new(r"(?<move_number>\d+)\.\s*(?<white_move>((O-O(-O)?)|(0-0(-0)?)|([a-h]((x[a-h][1-8])|[1-8])(=[QRBN])?)|([QRBN](([a-h][1-8]x?[a-h][1-8])|([1-8]x?[a-h][1-8])|([a-h]x?[a-h][1-8])|(x?[a-h][1-8])))|(Kx?[a-h][1-8]))\+?)\s+(?<black_move>((O-O(-O)?)|(0-0(-0)?)|([a-h]((x[a-h][1-8])|[1-8])(=[QRBN])?)|([QRBN](([a-h][1-8]x?[a-h][1-8])|([1-8]x?[a-h][1-8])|([a-h]x?[a-h][1-8])|(x?[a-h][1-8])))|(Kx?[a-h][1-8]))[+#]?)").unwrap();
        let halfmove_san_regex = Regex::new(r"(?<move_number>\d+)\.\s*(?<halfmove>((O-O(-O)?)|(0-0(-0)?)|([a-h]((x[a-h][1-8])|[1-8])(=[QRBN])?)|([QRBN](([a-h][1-8]x?[a-h][1-8])|([1-8]x?[a-h][1-8])|([a-h]x?[a-h][1-8])|(x?[a-h][1-8])))|(Kx?[a-h][1-8]))[+#]?)(\s*$|\s+\d)").unwrap();
        let result_regex = Regex::new(r"^(\n|.)*(?<white_score>0|1\/2|1)-(?<black_score>0|1\/2|1)\s*$").unwrap();
        let mut tokens = Vec::new();
        for caps in tag_pair_regex.captures_iter(text) {
            tokens.push(Token::TagPair(caps["name"].to_string(), caps["value"].replace(r"\\", r"\").replace(r#"\""#, r#"""#).to_string()));
        }
        for caps in fullmove_san_regex.captures_iter(text) {
            tokens.push(Token::FullmoveSan(caps["move_number"].parse().unwrap(), caps["white_move"].to_string(), caps["black_move"].to_string()));
        }
        for caps in halfmove_san_regex.captures_iter(text) {
            tokens.push(Token::HalfmoveSan(caps["move_number"].parse().unwrap(), caps["halfmove"].to_string()));
        }
        for caps in result_regex.captures_iter(text) {
            tokens.push(Token::Result(caps["white_score"].to_string(), caps["black_score"].to_string()));
        }
        tokens
    }

    /// Parses PGN from a collection of PGN tokens.
    /// This function currently does **not** support PGN annotations.
    fn parse(tokens: Vec<Token>) -> Result<Pgn, InvalidPgnError> {
        let mut tag_pairs_done = false;
        let mut fullmove_san_done = false;
        let mut halfmove_san_done = false;
        let mut result_done = false;
        let mut tag_pairs = HashMap::new();
        let mut moves = Vec::new();
        let mut result = None;
        for token in tokens {
            match token {
                Token::TagPair(name, value) => {
                    if tag_pairs_done || fullmove_san_done || halfmove_san_done || result_done {
                        return Err(InvalidPgnError::OrderOfElements("all tag pairs must be in the beginning of the text".to_owned()));
                    }
                    tag_pairs.insert(name, value);
                }
                Token::FullmoveSan(n, w, b) => {
                    if n < 1 {
                        return Err(InvalidPgnError::InvalidMoveNumber);
                    }
                    if fullmove_san_done || halfmove_san_done || result_done {
                        return Err(InvalidPgnError::NoAnnotations);
                    }
                    if !tag_pairs_done {
                        tag_pairs_done = true;
                    }
                    if let Some((prevn, _, _)) = moves.last() {
                        if *prevn != n - 1 {
                            return Err(InvalidPgnError::InvalidMoveNumber);
                        }
                    }
                    moves.push((n, Some(w), Some(b)));
                }
                Token::HalfmoveSan(n, w) => {
                    if n < 1 {
                        return Err(InvalidPgnError::InvalidMoveNumber);
                    }
                    if halfmove_san_done || result_done {
                        return Err(InvalidPgnError::NoAnnotations);
                    }
                    if !fullmove_san_done {
                        fullmove_san_done = true;
                    }
                    if let Some((prevn, _, _)) = moves.last() {
                        if *prevn != n - 1 {
                            return Err(InvalidPgnError::InvalidMoveNumber);
                        }
                    }
                    moves.push((n, Some(w), None));
                }
                Token::Result(w, b) => {
                    if !halfmove_san_done {
                        halfmove_san_done = true;
                    }
                    if result_done {
                        return Err(InvalidPgnError::OrderOfElements("there can only be one game result".to_owned()));
                    }
                    result_done = true;
                    result = Some((w, b));
                }
            }
        }
        if SEVEN_TAG_ROSTER.iter().any(|&k| !tag_pairs.contains_key(k)) {
            return Err(InvalidPgnError::SevenTagRoster);
        }
        let mut board = match tag_pairs.get("FEN") {
            Some(fen) => Board::from_fen(Fen::try_from(fen.as_str()).unwrap()),
            _ => Board::default(),
        };
        for (_, w, b) in moves {
            if let Some(m) = w {
                board.make_move_san(&m).map_err(|e| InvalidPgnError::InvalidMove(e))?;
            }
            if let Some(m) = b {
                board.make_move_san(&m).map_err(|e| InvalidPgnError::InvalidMove(e))?;
            }
        }
        match board.game_result() {
            Some(GameResult::Wins(Color::White, _)) => {
                if result != Some(("1".to_owned(), "0".to_owned())) {
                    return Err(InvalidPgnError::InvalidResult("white has won on the board but the result is not 1-0".to_owned()));
                }
            }
            Some(GameResult::Wins(Color::Black, _)) => {
                if result != Some(("0".to_owned(), "1".to_owned())) {
                    return Err(InvalidPgnError::InvalidResult("black has won on the board but the result is not 0-1".to_owned()));
                }
            }
            Some(GameResult::Draw(_)) => {
                if result != Some(("1/2".to_owned(), "1/2".to_owned())) {
                    return Err(InvalidPgnError::InvalidResult("the game has been drawn but the result is not 1/2-1/2".to_owned()));
                }
            }
            None => {
                if let Some(res) = result {
                    match (res.0.as_str(), res.1.as_str()) {
                        ("1", "0") => board.resign(Color::Black).unwrap(),
                        ("0", "1") => board.resign(Color::White).unwrap(),
                        ("1/2", "1/2") => board.agree_draw().unwrap(),
                        _ => return Err(InvalidPgnError::InvalidResult(format!("{}-{} is not a valid result", res.0, res.1))),
                    }
                }
            }
        }
        Ok(Self { tag_pairs, board })
    }

    /// Constructs a `Pgn` object from a `Board`.
    /// Tag pairs must be provided, following the Seven Tag Roster (<https://en.wikipedia.org/wiki/Portable_Game_Notation#Seven_Tag_Roster>),
    /// except the _Result_ tag which will be retrieved from the game state.
    pub fn from_board(board: Board, tag_pairs: Vec<(String, String)>) -> Result<Self, InvalidPgnError> {
        let tag_pair_names = tag_pairs.iter().map(|(t, _)| t.as_str()).collect::<Vec<_>>();
        let mut required_tags = SEVEN_TAG_ROSTER.iter().take(6);
        if required_tags.any(|tag| !tag_pair_names.contains(tag)) {
            return Err(InvalidPgnError::SevenTagRoster);
        }
        let mut tag_pairs_hm = HashMap::new();
        for (name, value) in tag_pairs.into_iter() {
            tag_pairs_hm.insert(name, value);
        }
        Ok(Self { board, tag_pairs: tag_pairs_hm })
    }

    /// Returns the PGN's tag pairs.
    pub fn tag_pairs(&self) -> &HashMap<String, String> {
        &self.tag_pairs
    }

    /// Returns the game that the PGN represents.
    pub fn board(&self) -> &Board {
        &self.board
    }
}

impl TryFrom<&str> for Pgn {
    type Error = InvalidPgnError;

    /// Attempts to parse a PGN text, returning an error if it is invalid.
    /// This function does **not** support PGN annotations.
    /// Note that this function is not a PGN validator, meaning it may sometimes accept invalid PGN as valid.
    fn try_from(text: &str) -> Result<Pgn, Self::Error> {
        Self::parse(Self::tokenize(text))
    }
}

impl fmt::Display for Pgn {
    /// Represents the `Pgn` object as PGN text.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut pgn = String::new();
        let mut tag_pairs = self.tag_pairs.clone();
        tag_pairs.insert("FEN".to_owned(), self.board.initial_fen().to_string());
        for &name in &SEVEN_TAG_ROSTER {
            tag_pairs.remove(name);
            let line = format!(r#"[{name} "{}"]{}"#, self.tag_pairs.get(name).unwrap(), "\n");
            pgn.push_str(&line);
        }
        let mut names: Vec<_> = tag_pairs.keys().collect();
        names.sort();
        for name in names {
            let line = format!(r#"[{name} "{}"]{}"#, self.tag_pairs.get(name).unwrap(), "\n");
            pgn.push_str(&line);
        }
        pgn.push('\n');
        pgn.push_str(&self.board.gen_movetext());
        pgn.push_str(&format!(
            " {}",
            match self.board.game_result() {
                Some(res) => res.to_string(),
                None => "*".to_owned(),
            }
        ));
        write!(f, "{pgn}")
    }
}

/// Represents a PGN token.
#[derive(Eq, PartialEq, Clone, Debug)]
enum Token {
    TagPair(String, String),
    FullmoveSan(usize, String, String),
    HalfmoveSan(usize, String),
    Result(String, String),
}

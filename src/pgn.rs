use regex::Regex;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Pgn {
    tag_pairs: HashMap<String, String>,
}

impl Pgn {
    /// Tokenizes PGN text.
    fn tokenize(text: &str) -> Vec<Token> {
        let text = text.replace('\n', "");
        let mut ptr = 0;
        let mut tokens = Vec::new();
        let mut token_range = [0, 0];
        while ptr < text.len() {
            dbg!(ptr, text.len());
            ptr += 1;
            let tok = text.chars().skip(token_range[0]).take(token_range[1] - token_range[0] + 1).collect::<String>();
            if let Some(token) = Token::from_str(&tok) {
                tokens.push(token);
                token_range = [ptr + 1, ptr + 1];
            } else {
                token_range[1] += 1;
            }
        }
        tokens
    }
    pub fn parse(pgn: &str) -> Result<Pgn, String> {
        let tokens = Self::tokenize(pgn);
        println!("{tokens:?}");
        todo!()
    }
}

/// Represents a PGN token.
#[derive(Eq, PartialEq, Clone, Debug)]
enum Token {
    TagPair(String, String),
    FullmoveSan(usize, String, String),
    HalfmoveSan(usize, String, String),
    Result(String, String),
}

impl Token {
    /// Attempts to convert part of a PGN text into a `Token`, returning `None` if not possible.
    fn from_str(string: &str) -> Option<Token> {
        let tag_pair_regex = Regex::new(r#"\[(?<name>[A-Za-z]+) *(?<value>"((\\\\)|(\\")|[^"\\])*")\]"#).unwrap();
        let fullmove_san_regex = Regex::new(r"(?<move_number>\d+)\. *(?<white_move>((O-O(-O)?)|(0-0(-0)?)|([a-h]((x[a-h][1-8])|[1-8]))|([QRBN](([a-h][1-8]x?[a-h][1-8])|([1-8]x?[a-h][1-8])|([a-h]x?[a-h][1-8])|(x?[a-h][1-8])))|(Kx?[a-h][1-8]))\+?) (?<black_move>((O-O(-O)?)|(0-0(-0)?)|([a-h]((x[a-h][1-8])|[1-8]))|([QRBN](([a-h][1-8]x?[a-h][1-8])|([1-8]x?[a-h][1-8])|([a-h]x?[a-h][1-8])|(x?[a-h][1-8])))|(Kx?[a-h][1-8]))[+#]?)").unwrap();
        let halfmove_san_regex = Regex::new(r"(?<move_number>\d+)(?<dots>\.(\.{2})?) *(?<halfmove>((O-O(-O)?)|(0-0(-0)?)|([a-h]((x[a-h][1-8])|[1-8]))|([QRBN](([a-h][1-8]x?[a-h][1-8])|([1-8]x?[a-h][1-8])|([a-h]x?[a-h][1-8])|(x?[a-h][1-8])))|(Kx?[a-h][1-8]))[+#]?)").unwrap();
        let result_regex = Regex::new(r#"[^"](?<white_score>0|1\/2|1)-(?<black_score>0|1\/2|1)[^"]?"#).unwrap();
        if let Some(caps) = tag_pair_regex.captures(string) {
            return Some(Token::TagPair(caps["name"].to_string(), caps["value"].to_string()));
        }
        if let Some(caps) = fullmove_san_regex.captures(string) {
            return Some(Token::FullmoveSan(caps["move_number"].parse().unwrap(), caps["white_move"].to_string(), caps["black_move"].to_string()));
        }
        if let Some(caps) = halfmove_san_regex.captures(string) {
            return Some(Token::HalfmoveSan(caps["move_number"].parse().unwrap(), caps["dots"].to_string(), caps["halfmove"].to_string()));
        }
        if let Some(caps) = result_regex.captures(string) {
            return Some(Token::Result(caps["white_score"].to_string(), caps["black_score"].to_string()));
        }
        None
    }
}

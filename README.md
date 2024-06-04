# rschess [![Made with Rust](https://img.shields.io/badge/made_with-rust-blue?&logo=rust)](https://rust-lang.org) [![Crates.io Version](https://img.shields.io/crates/v/rschess?logo=rust)](https://crates.io/crates/rschess) [![Crates.io Total Downloads](https://img.shields.io/crates/d/rschess?logo=rust&link=https%3A%2F%2Fcrates.io%2Fcrates%2Frschess)](https://crates.io/crates/rschess) ![Crates.io License](https://img.shields.io/crates/l/rschess) ![GitHub lines of Rust code](https://tokei.rs/b1/github/Python3-8/rschess?category=code&type=Rust&style=flat) ![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/Python3-8/rschess)
A Rust chess library with the aim to be as feature-rich as possible

# Examples
```rs
use rschess::{Board, Color, Fen, Move, GameResult, WinType};

let mut board = Board::default();
assert_eq!(board.to_fen(), Fen::try_from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap());
assert!(board.is_ongoing()); // the game is ongoing
assert!(board.side_to_move().is_white()); // white's turn to move
board.make_move_uci("e2e4").unwrap(); // plays e2 to e4, i.e. 1. e4
assert!(board.side_to_move().is_black()); // black's turn to move
board.make_move_san("e5").unwrap(); // plays 1... e5
assert!(board.is_legal(board.san_to_move("f4").unwrap())); // confirms that 2. f4 is legal
assert!(board.is_legal(Move::from_uci("d2d4").unwrap())); // confirms that d2 to d4, i.e. 2. d4 is legal
assert!(board.san_to_move("Ne4").is_err()); // confirms that 2. Ne4 is invalid in this position
assert!(!board.is_legal(Move::from_uci("e1g1").unwrap())); // confirms that e1 to g1, i.e. 2. O-O is invalid
assert_eq!(board.halfmove_clock(), 0); // confirms that the halfmove clock has been reset (since the last move w
board.make_move_san("Nf3").unwrap(); // plays 2. Nf3
assert_eq!(board.halfmove_clock(), 1); // confirms that the halfmove clock has incremented (since 2. Nf3 was not
board.make_move_san("f6").unwrap(); // plays 2... f6
board.make_move_san("Nxe5").unwrap(); // plays 3. Nxe5
assert_eq!(board.halfmove_clock(), 0); // confirms that the halfmove clock has been reset (since the last move w
board.make_move_san("fxe5").unwrap(); // plays 3... fxe5
board.make_move_san("Qh5+").unwrap(); // plays 4. Qh5+
assert!(board.is_check()); // confirms that a side is in check
assert_eq!(board.checked_side(), Some(Color::Black)); // confirms that black is the side in check
assert_eq!(board.gen_legal_moves().len(), 2); // confirms that there are only two legal moves (4... g6 and 4...
board.make_move_uci("e8e7").unwrap(); // plays e8 to e7, i.e. 4... Ke7
assert_eq!(board.halfmove_clock(), 2); // confirms that the halfmove clock has incremented twice (since two half
board.make_move_uci("h5e5").unwrap(); // plays h5 to e5, i.e. 5. Qxe5+
assert_eq!(board.halfmove_clock(), 0); // confirms that the halfmove clock has been reset (since the last move w
board.make_move_san("Kf7").unwrap(); // plays 5... Kf7
board.make_move_san("Bc4+").unwrap(); // plays 6. Bc4+
board.make_move_san("Kg6").unwrap(); // plays 6... Kg6
board.make_move_san("Qf5+").unwrap(); // plays 7. Qf5+
assert_eq!(board.gen_legal_moves().len(), 1); // confirms that there is only one legal move
board.make_move_san("Kh6").unwrap(); // plays 7... Kh6
board.make_move_san("d4+").unwrap(); // plays 8. d4+ (discovered check by the bishop on c1)
assert!(board.is_check()); // confirms that a side is in check
board.make_move_san("g5").unwrap(); // plays 8... g5
board.make_move_san("h4").unwrap(); // plays 9. h4
board.make_move_san("Bg7").unwrap(); // plays 9... Bg7
board.make_move_san("hxg5#").unwrap(); // plays 10. hxg5#
assert!(board.is_game_over()); // confirms that the game is over
assert!(board.is_checkmate()); // confirms that a side has been checkmated
assert_eq!(board.game_result(), Some(GameResult::Wins(Color::White, WinType::Checkmate))); // confirms that white has won
assert_eq!(board.fullmove_number(), 10); // confirms that the current fullmove number is 10
assert_eq!(board.gen_legal_moves().len(), 0); // confirms that there are no legal moves because the game is over
```

# Aim
rschess is yet another chess library for Rust, with the aim of being as feature-rich as possible. At the moment, speed optimizations aren't my first priority, but I might get there someday. There are surprisingly very few Rust crates that offer enough features for use in applications related to chess. With rschess I aim to create a library that boasts features of:
* legal move generation
* parsing/generating FEN and PGN
* move history
* board status (ongoing/over, checkmate, types of draws including the rare fivefold repetition and the seventy-five-move rule)
* generating an image of the chessboard
* and maybe more!

# Progress
At the moment, rschess has:
* legal move generation
* parsing/generating FEN
* parsing/interpreting/generating UCI and SAN
* board status
* pretty-printing

# History
A while ago I was looking to write a simple Rust program that simulates chess games. I'd used Python's [chess](https://pypi.org/project/chess) library before, and knew that my task would be very easy, if Rust had a similar crate. It didn't. I soon found myself scrolling through hundreds of potential options on Crates.io, just to find nothing useful. Therefore, I [asked on the Rust subreddit](https://www.reddit.com/r/rust/comments/1d0f6ou/is_there_a_good_chess_library_for_rust/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button) hoping someone would tell me about a powerful crate that no one has ever heard of lol. Of course, none of the answers were very helpful in finding a suitable crate, but [u/LePfeiff's comment](https://www.reddit.com/r/rust/comments/1d0f6ou/comment/l5mr1qg/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button)
> Sounds like a good contribution opportunity 😉 be the change you want to see

made me think. Initially I was hesitant, but with [encouragement from u/howtokillafox](https://www.reddit.com/r/rust/comments/1d0f6ou/comment/l5mzdw8/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button), the rschess project was born.

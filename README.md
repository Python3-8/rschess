# rschess [![Made with Rust](https://img.shields.io/badge/made_with-rust-blue?&logo=rust)](https://rust-lang.org) [![Crates.io Version](https://img.shields.io/crates/v/rschess?logo=rust)](https://crates.io/crates/rschess) [![Crates.io Total Downloads](https://img.shields.io/crates/d/rschess?logo=rust&link=https%3A%2F%2Fcrates.io%2Fcrates%2Frschess)](https://crates.io/crates/rschess) ![Crates.io License](https://img.shields.io/crates/l/rschess) ![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/Python3-8/rschess)

# Aim
rschess is yet another chess library for Rust, with the aim of being as feature-rich as possible. At the moment, speed optimizations aren't my first priority, but I might get there someday. There are surprisingly very few Rust crates that offer enough features for any application related to chess. With rschess I aim to create a library that boasts features of:
* legal move generation
* parsing/generating FEN and PGN
* move history
* board status (ongoing/over, checkmate, types of draws including the rare fivefold repetition and the seventy-five-move rule)
* generating an image of the chessboard
* and maybe more!

# Progress
As of now (May 31, 2024) rschess has:
* legal move generation
* parsing/generating FEN
* parsing UCI
* move history
* board status
* pretty-printing

# History
A while ago I was looking to write a simple Rust program that simulates chess games. I'd used Python's [chess](https://pypi.org/project/chess) library before, and knew that my task would be very easy, if Rust had a similar crate. It didn't. I soon found myself scrolling through hundreds of potential options on Crates.io, just to find nothing useful. Therefore, I [asked on the Rust subreddit](https://www.reddit.com/r/rust/comments/1d0f6ou/is_there_a_good_chess_library_for_rust/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button) hoping someone would tell me about a powerful crate that no one has ever heard of lol. Of course, none of the answers were very helpful in finding a suitable crate, but [u/LePfeiff's comment](https://www.reddit.com/r/rust/comments/1d0f6ou/comment/l5mr1qg/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button)
> Sounds like a good contribution opportunity 😉 be the change you want to see

made me think. Initially I was hesitant, but with [encouragement from u/howtokillafox](https://www.reddit.com/r/rust/comments/1d0f6ou/comment/l5mzdw8/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button), rschess project was born.

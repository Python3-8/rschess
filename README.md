# rschess [![Made with Rust](https://img.shields.io/badge/made_with-rust-blue?&logo=rust)](https://rust-lang.org) [![Crates.io Version](https://img.shields.io/crates/v/rschess?logo=rust)](https://crates.io/crates/rschess) [![Crates.io Total Downloads](https://img.shields.io/crates/d/rschess?logo=rust&link=https%3A%2F%2Fcrates.io%2Fcrates%2Frschess)](https://crates.io/crates/rschess) ![Crates.io License](https://img.shields.io/crates/l/rschess) ![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/Python3-8/rschess)
A Rust chess library with the aim to be as feature-rich as possible

## Table of Contents
* [Aim](#aim)
* [Features](#features)
  * [Parsing FEN](#parsing-fen)
  * [Parsing PGN](#parsing-pgn)
  * [Generating legal moves](#generating-legal-moves)
  * [Making moves](#making-moves)
  * [Board status](#board-status)
  * [Generating FEN](#generating-fen)
  * [Generating PGN](#generating-pgn)
    * [From PGN text](#from-pgn-text)
    * [From a board](#from-a-board)
  * [Pretty-printing](#pretty-printing)
  * [Position to image](#position-to-image)
    * [Image properties](#image-properties)
    * [Custom piece sets](#custom-piece-sets)
* [History](#history)
## Aim
This project aims to be as feature-rich as possible, **at the cost of performance (this may change in the future)**. There are surprisingly very few Rust crates that offer enough features for use in applications related to chess. With rschess I strive to create a library that offers all the necessary functionalities for the development of chess software.
## Features
### Parsing FEN
```rust
use rschess::{Board, Fen};

let board = Board::from_fen(Fen::try_from("2R5/4bppk/1p1p3Q/5R1P/4P3/5P2/r4q1P/7K b - - 6 50").unwrap());
let starting_position = Board::default(); // equivalent to Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".try_into().unwrap())
```
### Parsing PGN
To use PGN, you must first enable the `pgn` feature in `Cargo.toml`:
```toml
[dependencies]
rschess = { git = "https://github.com/Python3-8/rschess.git", features = ["pgn"] }
```
<details>
  <summary><em>Carlsen-Karjakin_WCC2016_R13_4.pgn</em></summary>

  ```pgn
  [Event "Carlsen - Karjakin World Championship Match"]
  [Site "New York, NY USA"]
  [Date "2016.11.30"]
  [EventDate "2016.11.11"]
  [Round "13.4"]
  [Result "1-0"]
  [White "Magnus Carlsen"]
  [Black "Sergey Karjakin"]
  [ECO "B54"]
  [WhiteElo "?"]
  [BlackElo "?"]
  [PlyCount "99"]

  1.e4 c5 2.Nf3 d6 3.d4 cxd4 4.Nxd4 Nf6 5.f3 e5 6.Nb3 Be7 7.c4
  a5 8.Be3 a4 9.Nc1 O-O 10.Nc3 Qa5 11.Qd2 Na6 12.Be2 Nc5 13.O-O
  Bd7 14.Rb1 Rfc8 15.b4 axb3 16.axb3 Qd8 17.Nd3 Ne6 18.Nb4 Bc6
  19.Rfd1 h5 20.Bf1 h4 21.Qf2 Nd7 22.g3 Ra3 23.Bh3 Rca8 24.Nc2
  R3a6 25.Nb4 Ra5 26.Nc2 b6 27.Rd2 Qc7 28.Rbd1 Bf8 29.gxh4 Nf4
  30.Bxf4 exf4 31.Bxd7 Qxd7 32.Nb4 Ra3 33.Nxc6 Qxc6 34.Nb5 Rxb3
  35.Nd4 Qxc4 36.Nxb3 Qxb3 37.Qe2 Be7 38.Kg2 Qe6 39.h5 Ra3
  40.Rd3 Ra2 41.R3d2 Ra3 42.Rd3 Ra7 43.Rd5 Rc7 44.Qd2 Qf6 45.Rf5
  Qh4 46.Rc1 Ra7 47.Qxf4 Ra2+ 48.Kh1 Qf2 49.Rc8+ Kh7 50.Qh6+ 1-0
  ```
  Source: [Chessgames.com](https://www.chessgames.com/nodejs/game/viewGamePGN?text=1&gid=1848607)
</details>

```rust
use rschess::{Board, pgn::Pgn};

let pgn = Pgn::try_from(include_str!("Carlsen-Karjakin_WCC2016_R13_4.pgn")).unwrap();
let board = pgn.board(); // &Board
assert_eq!(Board::from_fen("2R5/4bppk/1p1p3Q/5R1P/4P3/5P2/r4q1P/7K b - - 6 50".try_into().unwrap()).position(), board.position());
```
In the above example, the two `Board`s are not equal but their `Position`s are, because `Pgn` recognizes using the _'1-0'_ in the
text that white has won by resignation, whereas `Fen` does not (because FEN text does not contain information about the game result).
### Generating legal moves
```rust
use rschess::Board;

let board = Board::from_fen("2R5/4bppk/1p1p3Q/5R1P/4P3/5P2/r4q1P/7K b - - 6 50".try_into().unwrap());
let legal_moves = board.gen_legal_moves();
assert_eq!(legal_moves.len(), 2);
```
Note that there are no legal moves when a game is over.
### Making moves
```rust
use rschess::{Board, Move};

let mut board = Board::default();

// move from UCI
board.make_move(Move::from_uci("e2e4").unwrap()).unwrap();
// or shortened:
board.make_move_uci("c7c5").unwrap();

// move from SAN
board.make_move(board.san_to_move("Nc3").unwrap()).unwrap();
// or shortened:
board.make_move_san("Nc6").unwrap();

// print all legal moves in UCI
println!("{:?}", board.gen_legal_moves().iter().map(Move::to_uci).collect::<Vec<_>>());

// print all legal moves in SAN
println!("{:?}", board.gen_legal_moves().into_iter().map(|m| board.move_to_san(m).unwrap()).collect::<Vec<_>>());
```
<details>
  <summary>Output</summary>

  ```rust
  ["a1b1", "d1e2", "d1f3", "d1g4", "d1h5", "e1e2", "f1e2", "f1d3", "f1c4", "f1b5", "f1a6", "g1e2", "g1f3", "g1h3", "a2a3", "a2a4", "b2b3", "b2b4", "d2d3", "d2d4", "f2f3", "f2f4", "g2g3", "g2g4", "h2h3", "h2h4", "c3a4", "c3b5", "c3d5", "c3e2", "c3b1", "e4e5"]
  ["Rb1", "Qe2", "Qf3", "Qg4", "Qh5", "Ke2", "Be2", "Bd3", "Bc4", "Bb5", "Ba6", "Nge2", "Nf3", "Nh3", "a3", "a4", "b3", "b4", "d3", "d4", "f3", "f4", "g3", "g4", "h3", "h4", "Na4", "Nb5", "Nd5", "Nce2", "Nb1", "e5"]
  ```
</details>

### Board status
```rust
use rschess::{Board, Color, GameResult, WinType};

let start_pos = Board::default();
assert!(start_pos.is_ongoing());
assert_eq!(start_pos.stalemated_side(), None);
assert_eq!(start_pos.game_result(), None);

let mut board = Board::from_fen("2R5/4bppk/1p1p3Q/5R1P/4P3/5P2/r4q1P/7K b - - 6 50".try_into().unwrap());
board.make_moves_san("gxh6 Rxf7#").unwrap();
assert!(board.is_game_over());
assert!(board.is_checkmate());
assert_eq!(board.checkmated_side(), Some(Color::Black));
assert_eq!(board.game_result(), Some(GameResult::Wins(Color::White, WinType::Checkmate)));
```
rschess detects:
* checkmate
* stalemate
* threefold repetition
* insufficient checkmating material
* the fifty-move rule
* fivefold repetition
* and the seventy-five-move rule.

Note that threefold repetition and the fifty-move rule do not immediately end the game, as these are types of draws that must be claimed by a player.
### Generating FEN
```rust
use rschess::Board;

let board = Board::default();
let fen = board.to_fen(); // returns a Fen object
assert_eq!(fen.to_string(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
```
### Generating PGN
Here too, to use PGN, you must first enable the `pgn` feature in `Cargo.toml`:
```toml
[dependencies]
rschess = { git = "https://github.com/Python3-8/rschess.git", features = ["pgn"] }
```
#### From PGN text
<details>
  <summary><em>M290-study.pgn</em></summary>

  ```pgn
  [Event "?"]
  [Site "?"]
  [Date "????.??.??"]
  [Round "?"]
  [White "?"]
  [Black "?"]
  [Result "1-0"]
  [SetUp "1"]
  [FEN "bBrb1B2/P1n1r2p/1Kp1Pb1p/2pk1P1p/5P2/1P2pP2/1pP1P3/1R4n1 w - - 0 1"]

  1. Rd1+ Bd4 2. c4+ Kd6 3. Rxg1 Bc3 4. Rd1+ Bd4 5. Ka5 Bb7 6. Ka4 Ba8 7. Ka3 Bb7
  8. Ka2 Ba8 9. Kb1 Bb7 10. Kc2 Ba8 11. Kd3 Bb7 12. Re1 Ba8 13. Rf1 Bb7 14. Rd1
  Ba8 15. Kc2 Bb7 16. Kb1 Ba8 17. Ka2 Bb7 18. Ka3 Ba8 19. Ka4 Bb7 20. Ka5 Ba8 21.
  Kb6 h4 22. Ka5 Bb7 23. Ka4 Ba8 24. Ka3 Bb7 25. Ka2 Ba8 26. Kb1 Bb7 27. Kc2 Ba8
  28. Kd3 Bb7 29. Rf1 Ba8 30. Re1 Bb7 31. Rd1 Ba8 32. Kc2 Bb7 33. Kb1 Ba8 34. Ka2
  Bb7 35. Ka3 Ba8 36. Ka4 Bb7 37. Ka5 Ba8 38. Kb6 h3 39. Ka5 Bb7 40. Ka4 Ba8 41.
  Ka3 Bb7 42. Ka2 Ba8 43. Kb1 Bb7 44. Kc2 Ba8 45. Kd3 Bb7 46. Rf1 Ba8 47. Re1 Bb7
  48. Rd1 Ba8 49. Kc2 Bb7 50. Kb1 Ba8 51. Ka2 Bb7 52. Ka3 Ba8 53. Ka4 Bb7 54. Ka5
  Ba8 55. Kb6 h2 56. Ka5 Bb7 57. Ka4 Ba8 58. Ka3 Bb7 59. Ka2 Ba8 60. Kb1 Bb7 61.
  Kc2 Ba8 62. Kd3 Bb7 63. Rf1 Ba8 64. Re1 Bb7 65. Rd1 Ba8 66. Kc2 Bb7 67. Kb1 Ba8
  68. Ka2 Bb7 69. Ka3 Ba8 70. Ka4 Bb7 71. Ka5 Ba8 72. Kb6 h5 73. Ka5 Bb7 74. Ka4
  Ba8 75. Ka3 Bb7 76. Ka2 Ba8 77. Kb1 Bb7 78. Kc2 Ba8 79. Kd3 Bb7 80. Rf1 Ba8 81.
  Re1 Bb7 82. Rd1 Ba8 83. Kc2 Bb7 84. Kb1 Ba8 85. Ka2 Bb7 86. Ka3 Ba8 87. Ka4 Bb7
  88. Ka5 Ba8 89. Kb6 h4 90. Ka5 Bb7 91. Ka4 Ba8 92. Ka3 Bb7 93. Ka2 Ba8 94. Kb1
  Bb7 95. Kc2 Ba8 96. Kd3 Bb7 97. Rf1 Ba8 98. Re1 Bb7 99. Rd1 Ba8 100. Kc2 Bb7
  101. Kb1 Ba8 102. Ka2 Bb7 103. Ka3 Ba8 104. Ka4 Bb7 105. Ka5 Ba8 106. Kb6 h3
  107. Ka5 Bb7 108. Ka4 Ba8 109. Ka3 Bb7 110. Ka2 Ba8 111. Kb1 Bb7 112. Kc2 Ba8
  113. Kd3 Bb7 114. Rf1 Ba8 115. Re1 Bb7 116. Rd1 Ba8 117. Kc2 Bb7 118. Kb1 Ba8
  119. Ka2 Bb7 120. Ka3 Ba8 121. Ka4 Bb7 122. Ka5 Ba8 123. Kb6 h6 124. Ka5 Bb7
  125. Ka4 Ba8 126. Ka3 Bb7 127. Ka2 Ba8 128. Kb1 Bb7 129. Kc2 Ba8 130. Kd3 Bb7
  131. Rf1 Ba8 132. Re1 Bb7 133. Rd1 Ba8 134. Kc2 Bb7 135. Kb1 Ba8 136. Ka2 Bb7
  137. Ka3 Ba8 138. Ka4 Bb7 139. Ka5 Ba8 140. Kb6 h5 141. Ka5 Bb7 142. Ka4 Ba8
  143. Ka3 Bb7 144. Ka2 Ba8 145. Kb1 Bb7 146. Kc2 Ba8 147. Kd3 Bb7 148. Rf1 Ba8
  149. Re1 Bb7 150. Rd1 Ba8 151. Kc2 Bb7 152. Kb1 Ba8 153. Ka2 Bb7 154. Ka3 Ba8
  155. Ka4 Bb7 156. Ka5 Ba8 157. Kb6 h4 158. Ka5 Bb7 159. Ka4 Ba8 160. Ka3 Bb7
  161. Ka2 Ba8 162. Kb1 Bb7 163. Kc2 Ba8 164. Kd3 Bb7 165. Rf1 Ba8 166. Re1 Bb7
  167. Rd1 Ba8 168. Kc2 Bb7 169. Kb1 Ba8 170. Ka2 Bb7 171. Ka3 Ba8 172. Ka4 Bb7
  173. Ka5 Ba8 174. Kb6 h1=Q 175. Rxh1 Bg7 176. Rd1+ Bd4 177. Ka5 Bb7 178. Ka4 Ba8
  179. Ka3 Bb7 180. Ka2 Ba8 181. Kb1 Bb7 182. Kc2 Ba8 183. Kd3 Bb7 184. Rf1 Ba8
  185. Re1 Bb7 186. Rd1 Ba8 187. Kc2 Bb7 188. Kb1 Ba8 189. Ka2 Bb7 190. Ka3 Ba8
  191. Ka4 Bb7 192. Ka5 Ba8 193. Kb6 h2 194. Ka5 Bb7 195. Ka4 Ba8 196. Ka3 Bb7
  197. Ka2 Ba8 198. Kb1 Bb7 199. Kc2 Ba8 200. Kd3 Bb7 201. Rf1 Ba8 202. Re1 Bb7
  203. Rd1 Ba8 204. Kc2 Bb7 205. Kb1 Ba8 206. Ka2 Bb7 207. Ka3 Ba8 208. Ka4 Bb7
  209. Ka5 Ba8 210. Kb6 h3 211. Ka5 Bb7 212. Ka4 Ba8 213. Ka3 Bb7 214. Ka2 Ba8
  215. Kb1 Bb7 216. Kc2 Ba8 217. Kd3 Bb7 218. Rf1 Ba8 219. Re1 Bb7 220. Rd1 Ba8
  221. Kc2 Bb7 222. Kb1 Ba8 223. Ka2 Bb7 224. Ka3 Ba8 225. Ka4 Bb7 226. Ka5 Ba8
  227. Kb6 h1=Q 228. Rxh1 Bg7 229. Rd1+ Bd4 230. Ka5 Bb7 231. Ka4 Ba8 232. Ka3 Bb7
  233. Ka2 Ba8 234. Kb1 Bb7 235. Kc2 Ba8 236. Kd3 Bb7 237. Rf1 Ba8 238. Re1 Bb7
  239. Rd1 Ba8 240. Kc2 Bb7 241. Kb1 Ba8 242. Ka2 Bb7 243. Ka3 Ba8 244. Ka4 Bb7
  245. Ka5 Ba8 246. Kb6 h2 247. Ka5 Bb7 248. Ka4 Ba8 249. Ka3 Bb7 250. Ka2 Ba8
  251. Kb1 Bb7 252. Kc2 Ba8 253. Kd3 Bb7 254. Rf1 Ba8 255. Re1 Bb7 256. Rd1 Ba8
  257. Kc2 Bb7 258. Kb1 Ba8 259. Ka2 Bb7 260. Ka3 Ba8 261. Ka4 Bb7 262. Ka5 Ba8
  263. Kb6 h1=Q 264. Rxh1 Bg7 265. Rd1+ Bd4 266. Ka5 Bb7 267. Ka4 Ba8 268. Ka3 Bb7
  269. Ka2 Ba8 270. Kb1 Bb7 271. Kc2 Ba8 272. Kd3 Bb7 273. Rf1 Ba8 274. Re1 Bb7
  275. Rd1 Ba8 276. Kc2 Bb7 277. Kb1 Ba8 278. Ka2 Bb7 279. Ka3 Ba8 280. Ka4 Bb7
  281. Ka5 Ba8 282. Kb6 Bb7 283. Kxb7 b1=Q 284. Rxb1 Be5 285. Rd1+ Bd4 286. Rxd4+
  cxd4 287. Kb6 d3 288. a8=Q Rxb8+ 289. Qxb8 dxe2 290. Qxd8# 1-0
  ```
</details>

```rust
use rschess::pgn::Pgn;

let pgn = Pgn::try_from(include_str!("M290-study.pgn")).unwrap();
println!("{pgn}");
assert!(pgn.to_string().contains(&pgn.board().gen_movetext()));
```
`Board::gen_movetext` generates the SAN movetext of the game, excluding the game result. It is also important to note that PGN text must follow the [Seven Tag Roster](https://en.wikipedia.org/wiki/Portable_Game_Notation#Seven_Tag_Roster).
<details>
  <summary>Output</summary>

  ```pgn
  [Event "?"]
  [Site "?"]
  [Date "????.??.??"]
  [Round "?"]
  [White "?"]
  [Black "?"]
  [Result "1-0"]
  [FEN "bBrb1B2/P1n1r2p/1Kp1Pb1p/2pk1P1p/5P2/1P2pP2/1pP1P3/1R4n1 w - - 0 1"]
  [SetUp "1"]

  1. Rd1+ Bd4 2. c4+ Kd6 3. Rxg1 Bc3 4. Rd1+ Bd4 5. Ka5 Bb7 6. Ka4 Ba8 7. Ka3 Bb7 8. Ka2 Ba8 9. Kb1 Bb7 10. Kc2 Ba8 11. Kd3 Bb7 12. Re1 Ba8 13. Rf1 Bb7 14. Rd1 Ba8 15. Kc2 Bb7 16. Kb1 Ba8 17. Ka2 Bb7 18. Ka3 Ba8 19. Ka4 Bb7 20. Ka5 Ba8 21. Kb6 h4 22. Ka5 Bb7 23. Ka4 Ba8 24. Ka3 Bb7 25. Ka2 Ba8 26. Kb1 Bb7 27. Kc2 Ba8 28. Kd3 Bb7 29. Rf1 Ba8 30. Re1 Bb7 31. Rd1 Ba8 32. Kc2 Bb7 33. Kb1 Ba8 34. Ka2 Bb7 35. Ka3 Ba8 36. Ka4 Bb7 37. Ka5 Ba8 38. Kb6 h3 39. Ka5 Bb7 40. Ka4 Ba8 41. Ka3 Bb7 42. Ka2 Ba8 43. Kb1 Bb7 44. Kc2 Ba8 45. Kd3 Bb7 46. Rf1 Ba8 47. Re1 Bb7 48. Rd1 Ba8 49. Kc2 Bb7 50. Kb1 Ba8 51. Ka2 Bb7 52. Ka3 Ba8 53. Ka4 Bb7 54. Ka5 Ba8 55. Kb6 h2 56. Ka5 Bb7 57. Ka4 Ba8 58. Ka3 Bb7 59. Ka2 Ba8 60. Kb1 Bb7 61. Kc2 Ba8 62. Kd3 Bb7 63. Rf1 Ba8 64. Re1 Bb7 65. Rd1 Ba8 66. Kc2 Bb7 67. Kb1 Ba8 68. Ka2 Bb7 69. Ka3 Ba8 70. Ka4 Bb7 71. Ka5 Ba8 72. Kb6 h5 73. Ka5 Bb7 74. Ka4 Ba8 75. Ka3 Bb7 76. Ka2 Ba8 77. Kb1 Bb7 78. Kc2 Ba8 79. Kd3 Bb7 80. Rf1 Ba8 81. Re1 Bb7 82. Rd1 Ba8 83. Kc2 Bb7 84. Kb1 Ba8 85. Ka2 Bb7 86. Ka3 Ba8 87. Ka4 Bb7 88. Ka5 Ba8 89. Kb6 h4 90. Ka5 Bb7 91. Ka4 Ba8 92. Ka3 Bb7 93. Ka2 Ba8 94. Kb1 Bb7 95. Kc2 Ba8 96. Kd3 Bb7 97. Rf1 Ba8 98. Re1 Bb7 99. Rd1 Ba8 100. Kc2 Bb7 101. Kb1 Ba8 102. Ka2 Bb7 103. Ka3 Ba8 104. Ka4 Bb7 105. Ka5 Ba8 106. Kb6 h3 107. Ka5 Bb7 108. Ka4 Ba8 109. Ka3 Bb7 110. Ka2 Ba8 111. Kb1 Bb7 112. Kc2 Ba8 113. Kd3 Bb7 114. Rf1 Ba8 115. Re1 Bb7 116. Rd1 Ba8 117. Kc2 Bb7 118. Kb1 Ba8 119. Ka2 Bb7 120. Ka3 Ba8 121. Ka4 Bb7 122. Ka5 Ba8 123. Kb6 h6 124. Ka5 Bb7 125. Ka4 Ba8 126. Ka3 Bb7 127. Ka2 Ba8 128. Kb1 Bb7 129. Kc2 Ba8 130. Kd3 Bb7 131. Rf1 Ba8 132. Re1 Bb7 133. Rd1 Ba8 134. Kc2 Bb7 135. Kb1 Ba8 136. Ka2 Bb7 137. Ka3 Ba8 138. Ka4 Bb7 139. Ka5 Ba8 140. Kb6 h5 141. Ka5 Bb7 142. Ka4 Ba8 143. Ka3 Bb7 144. Ka2 Ba8 145. Kb1 Bb7 146. Kc2 Ba8 147. Kd3 Bb7 148. Rf1 Ba8 149. Re1 Bb7 150. Rd1 Ba8 151. Kc2 Bb7 152. Kb1 Ba8 153. Ka2 Bb7 154. Ka3 Ba8 155. Ka4 Bb7 156. Ka5 Ba8 157. Kb6 h4 158. Ka5 Bb7 159. Ka4 Ba8 160. Ka3 Bb7 161. Ka2 Ba8 162. Kb1 Bb7 163. Kc2 Ba8 164. Kd3 Bb7 165. Rf1 Ba8 166. Re1 Bb7 167. Rd1 Ba8 168. Kc2 Bb7 169. Kb1 Ba8 170. Ka2 Bb7 171. Ka3 Ba8 172. Ka4 Bb7 173. Ka5 Ba8 174. Kb6 h1=Q 175. Rxh1 Bg7 176. Rd1+ Bd4 177. Ka5 Bb7 178. Ka4 Ba8 179. Ka3 Bb7 180. Ka2 Ba8 181. Kb1 Bb7 182. Kc2 Ba8 183. Kd3 Bb7 184. Rf1 Ba8 185. Re1 Bb7 186. Rd1 Ba8 187. Kc2 Bb7 188. Kb1 Ba8 189. Ka2 Bb7 190. Ka3 Ba8 191. Ka4 Bb7 192. Ka5 Ba8 193. Kb6 h2 194. Ka5 Bb7 195. Ka4 Ba8 196. Ka3 Bb7 197. Ka2 Ba8 198. Kb1 Bb7 199. Kc2 Ba8 200. Kd3 Bb7 201. Rf1 Ba8 202. Re1 Bb7 203. Rd1 Ba8 204. Kc2 Bb7 205. Kb1 Ba8 206. Ka2 Bb7 207. Ka3 Ba8 208. Ka4 Bb7 209. Ka5 Ba8 210. Kb6 h3 211. Ka5 Bb7 212. Ka4 Ba8 213. Ka3 Bb7 214. Ka2 Ba8 215. Kb1 Bb7 216. Kc2 Ba8 217. Kd3 Bb7 218. Rf1 Ba8 219. Re1 Bb7 220. Rd1 Ba8 221. Kc2 Bb7 222. Kb1 Ba8 223. Ka2 Bb7 224. Ka3 Ba8 225. Ka4 Bb7 226. Ka5 Ba8 227. Kb6 h1=Q 228. Rxh1 Bg7 229. Rd1+ Bd4 230. Ka5 Bb7 231. Ka4 Ba8 232. Ka3 Bb7 233. Ka2 Ba8 234. Kb1 Bb7 235. Kc2 Ba8 236. Kd3 Bb7 237. Rf1 Ba8 238. Re1 Bb7 239. Rd1 Ba8 240. Kc2 Bb7 241. Kb1 Ba8 242. Ka2 Bb7 243. Ka3 Ba8 244. Ka4 Bb7 245. Ka5 Ba8 246. Kb6 h2 247. Ka5 Bb7 248. Ka4 Ba8 249. Ka3 Bb7 250. Ka2 Ba8 251. Kb1 Bb7 252. Kc2 Ba8 253. Kd3 Bb7 254. Rf1 Ba8 255. Re1 Bb7 256. Rd1 Ba8 257. Kc2 Bb7 258. Kb1 Ba8 259. Ka2 Bb7 260. Ka3 Ba8 261. Ka4 Bb7 262. Ka5 Ba8 263. Kb6 h1=Q 264. Rxh1 Bg7 265. Rd1+ Bd4 266. Ka5 Bb7 267. Ka4 Ba8 268. Ka3 Bb7 269. Ka2 Ba8 270. Kb1 Bb7 271. Kc2 Ba8 272. Kd3 Bb7 273. Rf1 Ba8 274. Re1 Bb7 275. Rd1 Ba8 276. Kc2 Bb7 277. Kb1 Ba8 278. Ka2 Bb7 279. Ka3 Ba8 280. Ka4 Bb7 281. Ka5 Ba8 282. Kb6 Bb7 283. Kxb7 b1=Q 284. Rxb1 Be5 285. Rd1+ Bd4 286. Rxd4+ cxd4 287. Kb6 d3 288. a8=Q Rxb8+ 289. Qxb8 dxe2 290. Qxd8# 1-0
  ```
</details>

#### From a board
The `Pgn` struct provides the `Pgn::from_board` method for creating `Pgn` objects using the moves on a `Board`.
```rust
use rschess::{Board, pgn::Pgn};

let mut board = Board::default();
board.make_moves_san("f3 e5 g4 Qh4#").unwrap();
let pgn = Pgn::from_board(
    board, // the board
    vec![  // PGN tag pairs
        ("Event", "?"),
        ("Site", "?"),
        ("Date", "????.??.??"),
        ("Round", "?"),
        ("White", "?"),
        ("Black", "?"),
    ]
    .into_iter()
    .map(|(t, v)| (t.to_owned(), v.to_owned()))
    .collect(),
)
.unwrap();
println!("{pgn}");
```
<details>
  <summary>Output</summary>

  ```pgn
  [Event "?"]
  [Site "?"]
  [Date "????.??.??"]
  [Round "?"]
  [White "?"]
  [Black "?"]
  [Result "0-1"]
  [FEN "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"]

  1. f3 e5 2. g4 Qh4# 0-1
  ```
</details>

In this example too, PGN must follow the [Seven Tag Roster](https://en.wikipedia.org/wiki/Portable_Game_Notation#Seven_Tag_Roster), with the exception of the _Result_ tag, because this will be determined from the status of the game on the `Board`.
### Pretty-printing
Pretty-printing the position from the perspective of the side whose turn it is to move:
```rust
use rschess::Board;

let board = Board::from_fen("2R5/4bppk/1p1p3Q/5R1P/4P3/5P2/r4q1P/7K b - - 6 50".try_into().unwrap());
println!("{board}");
```
<details>
  <summary>Output</summary>

  ```
  ‎  ┌───┬───┬───┬───┬───┬───┬───┬───┐
  1 │ ♔ │   │   │   │   │   │   │   │
  ‎  ├───┼───┼───┼───┼───┼───┼───┼───┤
  2 │ ♙ │   │ ♛ │   │   │   │   │ ♜ │
  ‎  ├───┼───┼───┼───┼───┼───┼───┼───┤
  3 │   │   │ ♙ │   │   │   │   │   │
  ‎  ├───┼───┼───┼───┼───┼───┼───┼───┤
  4 │   │   │   │ ♙ │   │   │   │   │
  ‎  ├───┼───┼───┼───┼───┼───┼───┼───┤
  5 │ ♙ │   │ ♖ │   │   │   │   │   │
  ‎  ├───┼───┼───┼───┼───┼───┼───┼───┤
  6 │ ♕ │   │   │   │ ♟ │   │ ♟ │   │
  ‎  ├───┼───┼───┼───┼───┼───┼───┼───┤
  7 │ ♚ │ ♟ │ ♟ │ ♝ │   │   │   │   │
  ‎  ├───┼───┼───┼───┼───┼───┼───┼───┤
  8 │   │   │   │   │   │ ♖ │   │   │
  ‎  └───┴───┴───┴───┴───┴───┴───┴───┘
  ‎  │ h │ g │ f │ e │ d │ c │ b │ a
  ```
</details>

Pretty-printing the position from the perspective of a specific side:
```rust
use rschess::{Board, Color};

let board = Board::from_fen("2R5/4bppk/1p1p3Q/5R1P/4P3/5P2/r4q1P/7K b - - 6 50".try_into().unwrap());
println!("{}", board.pretty_print(Color::White));
```
<details>
  <summary>Output</summary>

  ```
  ‎  ┌───┬───┬───┬───┬───┬───┬───┬───┐
  8 │   │   │ ♖ │   │   │   │   │   │
  ‎  ├───┼───┼───┼───┼───┼───┼───┼───┤
  7 │   │   │   │   │ ♝ │ ♟ │ ♟ │ ♚ │
  ‎  ├───┼───┼───┼───┼───┼───┼───┼───┤
  6 │   │ ♟ │   │ ♟ │   │   │   │ ♕ │
  ‎  ├───┼───┼───┼───┼───┼───┼───┼───┤
  5 │   │   │   │   │   │ ♖ │   │ ♙ │
  ‎  ├───┼───┼───┼───┼───┼───┼───┼───┤
  4 │   │   │   │   │ ♙ │   │   │   │
  ‎  ├───┼───┼───┼───┼───┼───┼───┼───┤
  3 │   │   │   │   │   │ ♙ │   │   │
  ‎  ├───┼───┼───┼───┼───┼───┼───┼───┤
  2 │ ♜ │   │   │   │   │ ♛ │   │ ♙ │
  ‎  ├───┼───┼───┼───┼───┼───┼───┼───┤
  1 │   │   │   │   │   │   │   │ ♔ │
  ‎  └───┴───┴───┴───┴───┴───┴───┴───┘
  ‎  │ a │ b │ c │ d │ e │ f │ g │ h
  ```
</details>

### Position to image
To use this feature, you must first enable the `img` feature in `Cargo.toml`:
```toml
[dependencies]
rschess = { git = "https://github.com/Python3-8/rschess.git", features = ["img"] }
```
Now, the `position_to_image` function can be used like so:
```rust
use rschess::{Board, img};

let board = Board::from_fen("2R5/4bppk/1p1p3Q/5R1P/4P3/5P2/r4q1P/7K b - - 6 50".try_into().unwrap());
img::position_to_image(
    board.position(),                        // the position
    img::PositionImageProperties::default(), // image properties
    board.side_to_move(),                    // perspective
)
.unwrap()
.save("Carlsen-Karjakin_WCC2016_R13_4.png")
.unwrap();
```
<details>
  <summary><em>Carlsen-Karjakin_WCC2016_R13_4.png</em></summary>

  ![Carlsen-Karjakin_WCC2016_R13_4](https://github.com/Python3-8/rschess/assets/66139317/da93f0c2-eb52-453d-8e76-609eefc55167)
</details>

#### Image properties
rschess allows customization of the image generated by `position_to_image`, using the `PositionImageProperties` struct. Custom colors can be provided for the light squares and the dark squares, and the size of the image can also be set. As for piece sets, a set of 25 options is available. These are some of the piece sets [listed as free to use](https://github.com/lichess-org/lila/blob/master/COPYING.md#exceptions-free) by Lichess.org.

#### Custom piece sets
Images of positions are not limited to the 25 built-in piece sets. Custom [`RgbaImage`](https://docs.rs/image/latest/image/type.RgbaImage.html)s can be provided for each black and white piece. This can be done, for example, with a folder named _pieces_ containing files named like so:
```
pieces
├── bB.png
├── bK.png
├── bN.png
├── bP.png
├── bQ.png
├── bR.png
├── wB.png
├── wK.png
├── wN.png
├── wP.png
├── wQ.png
└── wR.png
```
```rust
use image;
use rschess::{img, Board, Color, Fen};
use std::collections::HashMap;

let board = Board::from_fen(Fen::try_from("8/1r6/8/6n1/5k2/1b6/3K3N/7Q b - - 0 1").unwrap());
let mut pip = img::PositionImageProperties::default();
let mut hm = HashMap::new();
for fname in std::fs::read_dir("pieces").unwrap() {
    let fname = fname.unwrap();
    // piece name (e.g. wK, bB, wP, etc.):
    let name = String::from_utf8_lossy(fname.file_name().to_string_lossy().as_bytes()).split('.').next().unwrap().to_owned();
    // piece image:
    let piece_img = image::open(fname.path()).unwrap();
    hm.insert(name, piece_img.into());
}
pip.piece_set = img::PieceSet::Custom(hm);
img::position_to_image(board.position(), pip, Color::Black).unwrap().save("dtz1033.png").unwrap();
```

## History
A while ago I was looking to write a simple Rust program that simulates chess games. I'd used Python's [chess](https://pypi.org/project/chess) library before, and knew that my task would be very easy, if Rust had a similar crate. It didn't. I soon found myself scrolling through hundreds of potential options on Crates.io, just to find nothing useful. Therefore, I [asked on the Rust subreddit](https://www.reddit.com/r/rust/comments/1d0f6ou/is_there_a_good_chess_library_for_rust/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button) hoping someone would tell me about a powerful crate that no one has ever heard of lol. Of course, none of the answers were very helpful in finding a suitable crate, but [u/LePfeiff's comment](https://www.reddit.com/r/rust/comments/1d0f6ou/comment/l5mr1qg/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button)
> Sounds like a good contribution opportunity 😉 be the change you want to see

made me think. Initially I was hesitant, but with [encouragement from u/howtokillafox](https://www.reddit.com/r/rust/comments/1d0f6ou/comment/l5mzdw8/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button), the rschess project was born.

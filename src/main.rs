fn main() {
    use rschess::{Board, Color};

    let board = Board::from_fen("2R5/4bppk/1p1p3Q/5R1P/4P3/5P2/r4q1P/7K b - - 6 50".try_into().unwrap());
    println!("{}", board.pretty_print(Color::White));
}

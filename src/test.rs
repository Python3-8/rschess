use super::{Board, Move, PieceType};

#[test]
fn default_board() {
    println!("{:?}", Board::default());
}

#[test]
fn valid_fen() {
    Board::from_fen("6k1/8/6K1/6P1/8/8/8/8 w - - 0 1").unwrap();
    Board::from_fen("k5rb/8/8/4P3/3p4/8/8/K5BR w Kk - 0 1").unwrap();
}

#[test]
#[should_panic]
fn invalid_fen() {
    // Board::from_fen("what").unwrap();
    // Board::from_fen("blafsd o fs o sdo d").unwrap();
    // Board::from_fen("rnbkkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RKBQKBNR w KQkq - 0 1").unwrap();
    // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQQBNR w KQkq - 0 1").unwrap();
    // Board::from_fen("rnbqkbnr/pppppppp/8p/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    // Board::from_fen("rnbqkbnr/pppppxpp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP-RNBQKBNR w KQkq - 0 1").unwrap();
    // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR lol KQkq - 0 1").unwrap();
    // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b ros - 0 1").unwrap();
    // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KKqk - 0 1").unwrap();
    // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq C6 0 1").unwrap();
    // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq c5 0 1").unwrap();
    // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq brr 0 1").unwrap();
    // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0.1 1").unwrap();
    // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 151 1").unwrap();
    // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 150 0").unwrap();
    // Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 150 bro").unwrap();
    // Board::from_fen("k5rb/8/8/4P3/3p4/8/8/K5BR w KQkq - 0 1").unwrap();
    Board::from_fen("6k1/8/6K1/6P1/8/8/8/6p1 w - - 0 1").unwrap();
}

#[test]
fn idx_sq_conversion() {
    assert_eq!(Board::sq_to_idx('f', '5'), 37);
    assert_eq!(Board::idx_to_sq(37), ('f', '5'));
    assert_eq!(Board::sq_to_idx('g', '2'), 14);
    assert_eq!(Board::idx_to_sq(14), ('g', '2'));
    assert_eq!(Board::sq_to_idx('c', '6'), 42);
    assert_eq!(Board::idx_to_sq(42), ('c', '6'));
}

#[test]
fn board_to_fen() {
    assert_eq!(Board::from_fen("6k1/8/6K1/6P1/8/8/8/8 w - - 0 1").unwrap().to_fen(), "6k1/8/6K1/6P1/8/8/8/8 w - - 0 1");
    assert_eq!(Board::from_fen("k5rb/8/8/4P3/3p4/8/8/K5BR w Kk - 0 1").unwrap().to_fen(), "k5rb/8/8/4P3/3p4/8/8/K5BR w Kk - 0 1");
    assert_eq!(Board::default().to_fen(), Board::default().to_fen());
}

#[test]
fn pseudolegal_moves() {
    let check = |board: Board, legal: &[Move]| {
        let moves = Board::gen_pseudolegal_moves(&board.content, &board.castling_rights, board.en_passant_target, board.side_to_move);
        assert_eq!(moves, legal);
    };
    let board = Board::default();
    let legal = [
        Move(1, 16, None),
        Move(1, 18, None),
        Move(6, 21, None),
        Move(6, 23, None),
        Move(8, 16, None),
        Move(8, 24, None),
        Move(9, 17, None),
        Move(9, 25, None),
        Move(10, 18, None),
        Move(10, 26, None),
        Move(11, 19, None),
        Move(11, 27, None),
        Move(12, 20, None),
        Move(12, 28, None),
        Move(13, 21, None),
        Move(13, 29, None),
        Move(14, 22, None),
        Move(14, 30, None),
        Move(15, 23, None),
        Move(15, 31, None),
    ];
    check(board, &legal);
    let board = Board::from_fen("1k6/3p4/1K6/2P5/8/8/8/8 b - - 0 1").unwrap();
    let legal = [
        Move(51, 43, None),
        Move(51, 35, None),
        Move(57, 58, None),
        Move(57, 56, None),
        Move(57, 49, None),
        Move(57, 50, None),
        Move(57, 48, None),
    ];
    check(board, &legal);
    let board = Board::from_fen("1k6/8/1K6/2Pp4/8/8/8/8 w - d6 0 2").unwrap();
    let legal = [
        Move(34, 42, None),
        Move(34, 43, None),
        Move(41, 42, None),
        Move(41, 40, None),
        Move(41, 49, None),
        Move(41, 33, None),
        Move(41, 48, None),
        Move(41, 50, None),
        Move(41, 32, None),
    ];
    check(board, &legal);
    let board = Board::from_fen("k7/3N4/K7/8/8/8/8/8 w - - 0 1").unwrap();
    let legal = [
        Move(40, 41, None),
        Move(40, 48, None),
        Move(40, 32, None),
        Move(40, 33, None),
        Move(40, 49, None),
        Move(51, 57, None),
        Move(51, 61, None),
        Move(51, 45, None),
        Move(51, 36, None),
        Move(51, 34, None),
        Move(51, 41, None),
    ];
    check(board, &legal);
    let board = Board::from_fen("k7/3P4/K7/8/8/8/8/8 w - - 0 1").unwrap();
    let legal = [
        Move(40, 41, None),
        Move(40, 48, None),
        Move(40, 32, None),
        Move(40, 33, None),
        Move(40, 49, None),
        Move(51, 59, Some(PieceType::Q)),
        Move(51, 59, Some(PieceType::R)),
        Move(51, 59, Some(PieceType::B)),
        Move(51, 59, Some(PieceType::N)),
    ];
    check(board, &legal);
    let board = Board::from_fen("K7/8/k7/8/8/8/7p/8 b - - 0 1").unwrap();
    let legal = [
        Move(15, 7, Some(PieceType::Q)),
        Move(15, 7, Some(PieceType::R)),
        Move(15, 7, Some(PieceType::B)),
        Move(15, 7, Some(PieceType::N)),
        Move(40, 41, None),
        Move(40, 48, None),
        Move(40, 32, None),
        Move(40, 33, None),
        Move(40, 49, None),
    ];
    check(board, &legal);
}

use super::helpers;
use super::{Board, Color, Move, Pgn, PieceType, SpecialMoveType};

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
    // Board::from_fen("6k1/8/6K1/6P1/8/8/8/6p1 w - - 0 1").unwrap();
    Board::from_fen("8/8/4k3/8/2K2N2/8/8/8 w - - 0 1").unwrap();
}

#[test]
fn idx_sq_conversion() {
    assert_eq!(helpers::sq_to_idx('f', '5'), 37);
    assert_eq!(helpers::idx_to_sq(37), ('f', '5'));
    assert_eq!(helpers::sq_to_idx('g', '2'), 14);
    assert_eq!(helpers::idx_to_sq(14), ('g', '2'));
    assert_eq!(helpers::sq_to_idx('c', '6'), 42);
    assert_eq!(helpers::idx_to_sq(42), ('c', '6'));
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
        let moves = board.position.gen_pseudolegal_moves();
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
        Move(34, 43, Some(SpecialMoveType::EnPassant)),
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
        Move(51, 59, Some(SpecialMoveType::Promotion(PieceType::Q))),
        Move(51, 59, Some(SpecialMoveType::Promotion(PieceType::R))),
        Move(51, 59, Some(SpecialMoveType::Promotion(PieceType::B))),
        Move(51, 59, Some(SpecialMoveType::Promotion(PieceType::N))),
    ];
    check(board, &legal);
    let board = Board::from_fen("K7/8/k7/8/8/8/7p/8 b - - 0 1").unwrap();
    let legal = [
        Move(15, 7, Some(SpecialMoveType::Promotion(PieceType::Q))),
        Move(15, 7, Some(SpecialMoveType::Promotion(PieceType::R))),
        Move(15, 7, Some(SpecialMoveType::Promotion(PieceType::B))),
        Move(15, 7, Some(SpecialMoveType::Promotion(PieceType::N))),
        Move(40, 41, None),
        Move(40, 48, None),
        Move(40, 32, None),
        Move(40, 33, None),
        Move(40, 49, None),
    ];
    check(board, &legal);
}

#[test]
fn legal_moves() {
    let check = |board: Board, legal: &[Move]| {
        let moves = board.gen_legal_moves();
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
    let legal = [Move(51, 43, None), Move(51, 35, None), Move(57, 58, None), Move(57, 56, None)];
    check(board, &legal);
    let board = Board::from_fen("1k6/8/1K6/2Pp4/8/8/8/8 w - d6 0 2").unwrap();
    let legal = [
        Move(34, 42, None),
        Move(34, 43, Some(SpecialMoveType::EnPassant)),
        Move(41, 42, None),
        Move(41, 40, None),
        Move(41, 33, None),
        Move(41, 32, None),
    ];
    check(board, &legal);
    let board = Board::from_fen("k7/3N4/K7/8/8/8/8/8 w - - 0 1").unwrap();
    let legal = [];
    check(board, &legal);
    let board = Board::from_fen("k7/3P4/K7/8/8/8/8/8 w - - 0 1").unwrap();
    let legal = [
        Move(40, 41, None),
        Move(40, 32, None),
        Move(40, 33, None),
        Move(51, 59, Some(SpecialMoveType::Promotion(PieceType::Q))),
        Move(51, 59, Some(SpecialMoveType::Promotion(PieceType::R))),
        Move(51, 59, Some(SpecialMoveType::Promotion(PieceType::B))),
        Move(51, 59, Some(SpecialMoveType::Promotion(PieceType::N))),
    ];
    check(board, &legal);
    let board = Board::from_fen("K7/8/k7/8/8/8/7p/8 b - - 0 1").unwrap();
    let legal = [
        Move(15, 7, Some(SpecialMoveType::Promotion(PieceType::Q))),
        Move(15, 7, Some(SpecialMoveType::Promotion(PieceType::R))),
        Move(15, 7, Some(SpecialMoveType::Promotion(PieceType::B))),
        Move(15, 7, Some(SpecialMoveType::Promotion(PieceType::N))),
        Move(40, 41, None),
        Move(40, 32, None),
        Move(40, 33, None),
    ];
    check(board, &legal);
    let board = Board::from_fen("8/8/8/8/8/4k3/4p3/4K2R w K - 0 1").unwrap();
    let legal = [
        Move(7, 6, None),
        Move(7, 5, None),
        Move(7, 15, None),
        Move(7, 23, None),
        Move(7, 31, None),
        Move(7, 39, None),
        Move(7, 47, None),
        Move(7, 55, None),
        Move(7, 63, None),
    ];
    check(board, &legal);
    let board = Board::from_fen("8/8/8/8/8/2b1kb2/3R4/4K2R w K - 0 1").unwrap();
    let legal = [
        Move(4, 5, None),
        Move(4, 6, Some(SpecialMoveType::CastlingKingside)),
        Move(7, 6, None),
        Move(7, 5, None),
        Move(7, 15, None),
        Move(7, 23, None),
        Move(7, 31, None),
        Move(7, 39, None),
        Move(7, 47, None),
        Move(7, 55, None),
        Move(7, 63, None),
    ];
    check(board, &legal);
}

#[test]
fn to_san() {
    let mut board = Board::from_fen("7k/4Q3/6Q1/3Q4/6Q1/8/2Q3Q1/K3Q3 w - - 0 1").unwrap();
    assert_eq!(board.move_to_san(Move::from_uci("g6e4").unwrap()).unwrap(), "Q6e4");
    board.make_move(Move::from_uci("g6e4").unwrap()).unwrap();
    assert_eq!(board.stalemated_side(), Some(Color::Black));
    let mut board = Board::from_fen("6B1/2N1N3/1N3N2/8/1N3N2/2N1N1K1/8/7k w - - 0 1").unwrap();
    assert_eq!(board.move_to_san(Move::from_uci("c7d5").unwrap()).unwrap(), "Nc7d5");
    assert_eq!(board.move_to_san(Move::from_uci("g8d5").unwrap()).unwrap(), "Bd5+");
    board.make_move_uci("g8d5").unwrap();
    board.make_move_uci("h1g1").unwrap();
    assert_eq!(board.move_to_san(Move::from_uci("f4h3").unwrap()).unwrap(), "Nh3#");
    board.make_move_uci("f4h3").unwrap();
    assert_eq!(board.checkmated_side(), Some(Color::Black));
}

#[test]
fn insufficient_material() {
    assert!(Board::from_fen("k1b1b1b1/1b1b1b1B/b1b1b1B1/1b1b1B1B/b1b1B1B1/1b1B1B1B/b1B3B1/1B1B1B1K w - - 0 1")
        .unwrap()
        .is_insufficient_material());
    assert!(!Board::from_fen("k1b1b1b1/1b1b1b1B/b1b1b1B1/1b1bbB1B/b1b1B1B1/1b1BBB1B/b1B3B1/1B1B1B1K w - - 0 1")
        .unwrap()
        .is_insufficient_material());
    assert!(!Board::from_fen("kn6/8/1K6/3N4/8/8/8/8 w - - 0 1").unwrap().is_insufficient_material());
    assert!(!Board::from_fen("kB6/8/bK6/8/8/8/8/8 w - - 0 1").unwrap().is_insufficient_material());
    assert!(Board::from_fen("k1B5/8/bK6/8/8/8/8/8 w - - 0 1").unwrap().is_insufficient_material());
    assert!(Board::from_fen("k1N5/8/1K6/8/8/8/8/8 w - - 0 1").unwrap().is_insufficient_material());
}

#[test]
#[should_panic]
fn invalid_make_move_san() {
    let mut board = Board::default();
    board.make_move_san("Nc3").unwrap();
    board.make_move_san("Nc6").unwrap();
    board.make_move_san("e3").unwrap();
    board.make_move_san("e6").unwrap();
    board.make_move_san("Ne2").unwrap();
}

#[test]
fn valid_make_move_san() {
    let mut board = Board::default();
    board.make_move_san("Nc3").unwrap();
    board.make_move_san("Nc6").unwrap();
    board.make_move_san("e3").unwrap();
    board.make_move_san("e6").unwrap();
    board.make_move_san("Nge2").unwrap();
    println!("{}", board.pretty_print(Color::Black));
}

#[test]
#[ignore]
fn pgn_tokenize() {
    let pgn_str = include_str!("../test1.pgn");
    let pgn = Pgn::try_from(pgn_str).unwrap();
    println!("{pgn}");
}

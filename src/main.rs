use chess::{Board, parse_str_move};

fn main() {

    let board = Board::new();
    println!("{}", board.as_string());
    let test_move = parse_str_move("pb2", &board).unwrap();
    board.move_piece(test_move);
    println!("{}", board.as_string());
}

use chess::{Board, parse_str_move};

fn main() {

    let mut board = Board::new();
    println!("{}", board.as_string());
    let test_move = parse_str_move("pb3", &board).unwrap();
    board.move_piece(test_move).expect("Failed to move?");
    println!("{}", board.as_string());
}

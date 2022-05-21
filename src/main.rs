use chess::{Board, parse_str_move};
use std::io;

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {

    let mut board = Board::new();
    let mut msg = String::new();
    loop {
        clear();
        println!("\n{}\n{}", board.as_string(), msg);
        msg.clear();
        println!("Enter your next move.\n>>>    ");
        let mut input_buffer: String = String::new();

        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Failed to read line");

        let trimmed = input_buffer.trim();

        match parse_str_move(trimmed, &board) {
            Ok(_move) => {
                board.move_piece(_move).expect("Failed to make the valid move?");
            },
            Err(err) => {
                msg = format!("Command invalid: {}", err);
            },
        }
    }
}

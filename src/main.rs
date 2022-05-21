use chess::{Board, parse_str_move};
use std::io;

// To enable terminal colours
#[cfg(windows)]
fn enable_virtual_terminal_processing() {
    use winapi_util::console::Console;

    if let Ok(mut term) = Console::stdout() {
        let _ = term.set_virtual_terminal_processing(true);
    }
    if let Ok(mut term) = Console::stderr() {
        let _ = term.set_virtual_terminal_processing(true);
    }
}

fn main() {
    enable_virtual_terminal_processing();

    let mut board = Board::new();
    let mut msg = String::new();
    let clear_string = format!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    loop {
        println!("{}\n{}{}",clear_string, board.as_string(), msg);
        msg.clear();
        println!("Enter your next move. Examples: nf2; ng0f2; pe2; pe3; etc");
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

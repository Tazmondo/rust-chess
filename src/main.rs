use std::collections::VecDeque;
use chess::{Board, GameState, parse_str_move};
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
    if cfg!(windows) {
        enable_virtual_terminal_processing();
    }

    let mut premoves = VecDeque::from(vec!["pe3","pe6","qh5","pe5","bc4","pe4"]);
    let mut premoves = VecDeque::new();

    let mut board = Board::new();
    let mut msg = String::new();
    let clear_string = format!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    loop {
        if cfg!(debug_assertions) {
            println!("\n{}{}", board.as_string(), msg);
        } else {
            println!("{}\n{}{}",clear_string, board.as_string(), msg);
        }
        msg.clear();
        println!("{:?} Player, enter your next move. Examples: nf3; ng1f3; pe3; pe4; etc", board.turn);

        let mut input_buffer: String = String::new();

        if !premoves.is_empty() {
            input_buffer.push_str(premoves.pop_front().unwrap());
        } else {
            io::stdin()
                .read_line(&mut input_buffer)
                .expect("Failed to read line");
        }

        let trimmed = input_buffer.trim();

        match parse_str_move(trimmed, &board) {
            Ok(_move) => {
                let move_result = board.move_piece(_move);
                match move_result {
                    Err(err) => msg = format!("Could not move: {}", err),
                    Ok(state) => match state {
                        GameState::Playing => {}
                        GameState::Checkmate(colour) => {
                            println!("{:?} wins!", !colour);
                            break
                        }
                        GameState::Stalemate => {
                            println!("Stalemate...");
                            break
                        }
                    }
                }
            },
            Err(err) => {
                msg = format!("Command invalid: {}", err);
            },
        }
    }

    // Don't immediately close the terminal
    io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");
}

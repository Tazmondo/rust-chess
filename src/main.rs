use std::collections::VecDeque;
use chess::{Board, Colour, Coord, GameState, parse_str_move};
use std::io;
use std::io::stdout;
use crossterm::{event};
use crossterm::event::MouseEvent;

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

    // Enable terminal mouse support
    crossterm::execute!(stdout(), event::EnableMouseCapture).unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();  // Re-enable ctrl c to quit

    const MOUSE_MODE: bool = true;

    let mut premoves = VecDeque::from(vec!["pe3","pe6","qh5","pe5","bc4","pe4"]);
    let mut premoves = VecDeque::new();

    let mut board = Board::new();
    let mut msg = String::new();

    let clear_string = format!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    if MOUSE_MODE {
        let mut start: Option<Coord> = None;

        println!("{}\n{}{}",clear_string, board.as_string(), msg);
        loop {
            if let Ok(event::Event::Mouse(MouseEvent {kind: event::MouseEventKind::Down(event::MouseButton::Left), row, column, ..})) = event::read() {
                if (3..=10).contains(&row) && (3..=24).contains(&column) {
                    let calculated_row = 10 - row;
                    let calculated_column = (column/3) - 1;

                    if start == None {
                        start = Some(Coord {
                            column: calculated_column as i32,
                            row: calculated_row as i32
                        });
                        msg.push_str("Selected...\n");
                    } else {
                        let start = start.take().unwrap();
                        let end = Coord {
                            row: calculated_row as i32,
                            column: calculated_column as i32
                        };

                        msg.push_str(&format!("{:?} {:?}", start, end));

                        let state = board.attempt_move_with_coords(start, end);

                        match state {
                            Err(err) => msg.push_str(&format!("Could not move: {}\n", err)),
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
                    }
                    println!("{}\n{}{}", clear_string, board.as_string(), msg);
                    msg.clear();
                }
            };
        }
    } else {
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
                    match board.move_piece(_move) {
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
    }

    // Don't immediately close the terminal
    io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");
}

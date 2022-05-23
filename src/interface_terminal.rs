use std::collections::VecDeque;
use chess::{Space, Board, ColourPiece, Coord, GameState, parse_str_move};
use chess::Piece::*;
use chess::Colour::*;
use std::io;
use std::io::stdout;
use ansi_term::{Colour as TermColour, Style};
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

fn string_board(board: &Board) -> String {
    let term_black = Style::new().on(TermColour::RGB(35, 35, 35)).fg(TermColour::White);
    let term_white = Style::new().on(TermColour::RGB(155, 155, 155)).fg(TermColour::Black);
    let term_other = Style::new().fg(TermColour::Green);

    let column_label = term_other.paint("   A  B  C  D  E  F  G  H \n");

    let mut board_string = String::with_capacity(128);
    // Add letters at top
    board_string.push_str(&format!("\n{}", column_label));

    board_string.push_str(&format!("{}", term_other.paint("1|")));
    board.pieces.iter().enumerate().for_each(|(index, piece)| {
        if index % 8 == 0 && index != 0 {
            board_string.push_str(&format!("\n{}", term_other.paint(format!("{}|", (index / 8) + 1))))
        } else if index == 0 {}

        let piece_char = match piece {
            Space::Empty => format!("{}", term_other.paint(" # ")),
            Space::Full(ColourPiece { variant: Pawn, colour: White }) => format!("{}", term_white.paint(" P ")),
            Space::Full(ColourPiece { variant: Pawn, colour: Black }) => format!("{}", term_black.paint(" P ")),
            Space::Full(ColourPiece { variant: Knight, colour: White }) => format!("{}", term_white.paint(" N ")),
            Space::Full(ColourPiece { variant: Knight, colour: Black }) => format!("{}", term_black.paint(" N ")),
            Space::Full(ColourPiece { variant: Bishop, colour: White }) => format!("{}", term_white.paint(" B ")),
            Space::Full(ColourPiece { variant: Bishop, colour: Black }) => format!("{}", term_black.paint(" B ")),
            Space::Full(ColourPiece { variant: Rook, colour: White }) => format!("{}", term_white.paint(" R ")),
            Space::Full(ColourPiece { variant: Rook, colour: Black }) => format!("{}", term_black.paint(" R ")),
            Space::Full(ColourPiece { variant: Queen, colour: White }) => format!("{}", term_white.paint(" Q ")),
            Space::Full(ColourPiece { variant: Queen, colour: Black }) => format!("{}", term_black.paint(" Q ")),
            Space::Full(ColourPiece { variant: King, colour: White }) => format!("{}", term_white.paint(" K ")),
            Space::Full(ColourPiece { variant: King, colour: Black }) => format!("{}", term_black.paint(" K ")),
        };
        board_string.push_str(&piece_char);

        if (index + 1) % 8 == 0 {
            board_string.push_str(&format!("{}", term_other.paint(format!("|{}", (index / 8) + 1))));
        }
    });


    // Terminal displays top to bottom, but board is bottom to top. So lines must be reversed
    // board_string = board_string.lines().rev().map(|line| String::from(line) + "\n").collect();

    //Add letters at bottom
    board_string.push_str(&format!("\n{}\n", column_label));

    board_string
}

pub fn start_terminal() {
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

        println!("{}\n{}{}",clear_string, string_board(&board), msg);
        loop {
            if let Ok(event::Event::Mouse(MouseEvent {kind: event::MouseEventKind::Down(event::MouseButton::Left), row, column, ..})) = event::read() {
                if (3..=10).contains(&row) && (3..=25).contains(&column) {
                    let calculated_row = row-3;
                    let calculated_column = ((column+1)/3) - 1;

                    if start == None {
                        start = Some(Coord {
                            column: calculated_column as i32,
                            row: calculated_row as i32
                        });
                        // Todo: validate that this is a valid starting piece
                        // todo: highlight this piece on the board string
                        msg.push_str("Selected...\n");
                    } else {
                        let start = start.take().unwrap();
                        let end = Coord {
                            row: calculated_row as i32,
                            column: calculated_column as i32
                        };

                        let state = board.attempt_move_with_coords(start, end);

                        match state {
                            Err(err) => msg.push_str(&format!("Could not move: {}\n", err)),
                            Ok(state) => match state {
                                GameState::Playing => {}
                                GameState::Checkmate(colour) => {
                                    println!("{}\n{}",clear_string, string_board(&board));
                                    println!("{:?} wins!", !colour);
                                    break
                                }
                                GameState::Stalemate => {
                                    println!("{}\n{}",clear_string, string_board(&board));
                                    println!("Stalemate...");
                                    break
                                }
                            }
                        }
                    }
                    println!("{}\n{}{}", clear_string, string_board(&board), msg);
                    println!("It's {:?}'s turn!\n", board.turn);
                    msg.clear();
                }
            };
        }
    } else {
        loop {
            if cfg!(debug_assertions) {
                println!("\n{}{}", string_board(&board), msg);
            } else {
                println!("{}\n{}{}",clear_string, string_board(&board), msg);
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
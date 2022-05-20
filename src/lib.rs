use ansi_term::{Colour as TermColour, Style};

pub enum Colour {
    White,
    Black
}

use Colour::*;

pub enum Piece {
    Empty,  // For empty spaces on the board
    Pawn(Colour),
    Knight(Colour),
    Bishop(Colour),
    Rook(Colour),
    Queen(Colour),
    King(Colour)
}

use Piece::*;

pub struct Move {
    piece: Piece,
    start: i32,
    end: i32
}

struct Square {
    column: i32,
    row: i32,
    index: i32
}

impl Square {
    fn from_index(index: i32) -> Square {
        if index < 0 || index > 63 {
            panic!("Invalid index passed: {}", index)
        }

        Square {
            column: index % 8,
            row: index / 8,
            index
        }
    }

    fn from_coord(row: i32, column: i32) -> Square {
        if row < 0 || row > 63 || column < 0 || column > 63 {
            panic!("Invalid row and/or column passed:\nrow: {}\ncolumn: {}", row, column)
        }

        Square {
            column,
            row,
            index: (row * 8)
        }
    }

    fn from_str(coords: &[char]) -> Result<Square, String> {
        let column = match &coords[1] {
            'a' | '0' => Ok(0),
            'b' | '1' => Ok(1),
            'c' | '2' => Ok(2),
            'd' | '3' => Ok(3),
            'e' | '4' => Ok(4),
            'f' | '5' => Ok(5),
            'g' | '6' => Ok(6),
            'h' | '7' => Ok(7),
            _ => Err("Invalid column letter")
        }?;
        let row = coords[2].to_digit(10).ok_or("Invalid row number")? as i32;

        let index = ((row-1) * 8) + column;

        Ok(Square::from_index(index))
    }
}

// Board indexes will start at bottom left.
pub struct Board {
    pub pieces: [Piece; 64],

    term_white: Style,
    term_black: Style
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            // Generated using python file
            pieces: [Rook(White), Knight(White), Bishop(White), Queen(White), King(White), Bishop(White), Knight(White), Rook(White), Pawn(White), Pawn(White), Pawn(White), Pawn(White), Pawn(White), Pawn(White), Pawn(White), Pawn(White), Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Pawn(Black), Pawn(Black), Pawn(Black), Pawn(Black), Pawn(Black), Pawn(Black), Pawn(Black), Pawn(Black), Rook(Black), Knight(Black), Bishop(Black), Queen(Black), King(Black), Bishop(Black), Knight(Black), Rook(Black)],

            term_white: Style::new().on(TermColour::Black).fg(TermColour::White),
            term_black: Style::new().on(TermColour::White).fg(TermColour::Black)
        }
    }

    pub fn as_string(&self) -> String {
        let mut board_string = String::with_capacity(64);

        self.pieces.iter().enumerate().for_each(|(index, piece)| {
            if index % 8 == 0 && index != 0 {
                board_string.push('\n');
            }

            let piece_char = match piece {
                Empty => format!("{}", TermColour::Green.paint("# ")),
                Pawn(White) => format!("{}", self.term_white.paint("P ")),
                Pawn(Black) => format!("{}", self.term_black.paint("P ")),
                Knight(White) => format!("{}", self.term_white.paint("N ")),
                Knight(Black) => format!("{}", self.term_black.paint("N ")),
                Bishop(White) => format!("{}", self.term_white.paint("B ")),
                Bishop(Black) => format!("{}", self.term_black.paint("B ")),
                Rook(White) => format!("{}", self.term_white.paint("R ")),
                Rook(Black) => format!("{}", self.term_black.paint("R ")),
                Queen(White) => format!("{}", self.term_white.paint("Q ")),
                Queen(Black) => format!("{}", self.term_black.paint("Q ")),
                King(White) => format!("{}", self.term_white.paint("K ")),
                King(Black) => format!("{}", self.term_black.paint("K ")),
            };
            board_string.push_str(&piece_char);
        });

        // Terminal displays top to bottom, but board is bottom to top. So lines must be reversed
        board_string.lines().rev().map(|line| String::from(line) + "\n").collect()
    }

    pub fn move_piece(move_string: &str) -> Result<(), &str> {
        Ok(())
    }
}

fn index_from_piece(piece: &Piece, desired_move: i32, board: &Board) {
    let _ = board.pieces.iter().enumerate().filter(|(index, value)| {
        matches!(value, piece)
    });
}

pub fn parse_strmove(move_string: &str, board: &Board) -> Result<Move, String> {
    if !(move_string.is_ascii()) {
        return Err(String::from("String passed was not ascii"))
    };

    let char_vec: Vec<char> = move_string.chars().collect();

    let rest = match char_vec.len() {
        3 => {
            let piece = char_vec[0];
            let index = Square::from_str(&char_vec[1..3])?;

            Ok(())
        },
        5 => {

            Ok(())
        },
        length => Err(String::from("Invalid length passed."))
    };

    Ok(Move {
        piece: Piece::Empty,
        start: 0,
        end: 0
    })
}

#[cfg(test)]
mod tests {
    use crate::Board;

    #[test]
    fn board() {
        let board = Board::new();
        println!("{}", board.as_string());
    }
}

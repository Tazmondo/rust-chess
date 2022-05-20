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
                Empty => format!("{}", TermColour::Green.paint("#")),
                Pawn(White) => format!("{}", self.term_white.paint("p")),
                Pawn(Black) => format!("{}", self.term_black.paint("p")),
                Knight(White) => format!("{}", self.term_white.paint("n")),
                Knight(Black) => format!("{}", self.term_black.paint("n")),
                Bishop(White) => format!("{}", self.term_white.paint("b")),
                Bishop(Black) => format!("{}", self.term_black.paint("b")),
                Rook(White) => format!("{}", self.term_white.paint("r")),
                Rook(Black) => format!("{}", self.term_black.paint("r")),
                Queen(White) => format!("{}", self.term_white.paint("q")),
                Queen(Black) => format!("{}", self.term_black.paint("q")),
                King(White) => format!("{}", self.term_white.paint("k")),
                King(Black) => format!("{}", self.term_black.paint("k")),
            };
            board_string.push_str(&piece_char);
        });

        board_string
    }
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

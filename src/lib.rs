use ansi_term::{Colour as TermColour, Style};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Colour {
    White,
    Black,
}

use Colour::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

use Piece::*;


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ColourPiece {
    variant: Piece,
    colour: Colour,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Space {
    Empty,
    Full(ColourPiece),
}

use Space::*;

impl ColourPiece {
    fn from_char(char: char, board: &Board) -> Option<ColourPiece> {
        match char {
            'p' => Some(ColourPiece { variant: Pawn, colour: board.turn }),
            'b' => Some(ColourPiece { variant: Bishop, colour: board.turn }),
            'n' => Some(ColourPiece { variant: Knight, colour: board.turn }),
            'r' => Some(ColourPiece { variant: Rook, colour: board.turn }),
            'q' => Some(ColourPiece { variant: Queen, colour: board.turn }),
            'k' => Some(ColourPiece { variant: King, colour: board.turn }),
            _ => None
        }
    }
}

// Represents an arbitrary coordinate
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Coord {
    row: i32,
    column: i32,
}

// Represents a coordinate that actually exists on the board
#[derive(PartialEq, Copy, Clone, Debug)]
struct Square {
    coord: Coord,
    index: i32,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Move {
    piece: ColourPiece,
    start: Square,
    end: Square,
}

impl Square {
    fn from_index(index: i32) -> Square {
        if !(0..=63).contains(&index) {
            panic!("Invalid index passed: {}", index)
        }

        Square {
            coord: Coord {
                column: index % 8,
                row: index / 8,
            },
            index,
        }
    }

    fn from_coord(coord: &Coord) -> Square {
        if coord.column < 0 || coord.column > 7 || coord.row < 0 || coord.row > 7 {
            panic!("Invalid row and/or column passed:\nrow: {}\ncolumn: {}", coord.row, coord.column)
        }

        Square {
            index: (coord.row * 8) + coord.column,
            coord: *coord,
        }
    }

    fn from_str(coords: &[char]) -> Result<Square, String> {
        let column = match &coords[0] {
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
        let row = coords[1].to_digit(10).ok_or("Row was not a number")? as i32;
        if !(0..=7).contains(&row) {
            return Err(String::from("Row was not 0 to 7"));
        }

        let index = (row * 8) + column;

        Ok(Square::from_index(index))
    }
}

// Board indexes will start at bottom left.
pub struct Board {
    pub pieces: [Space; 64],
    pub turn: Colour,

    term_white: Style,
    term_black: Style,
    term_other: Style
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
            pieces: [Full(ColourPiece { variant: Rook, colour: White }), Full(ColourPiece { variant: Knight, colour: White }), Full(ColourPiece { variant: Bishop, colour: White }), Full(ColourPiece { variant: Queen, colour: White }), Full(ColourPiece { variant: King, colour: White }), Full(ColourPiece { variant: Bishop, colour: White }), Full(ColourPiece { variant: Knight, colour: White }), Full(ColourPiece { variant: Rook, colour: White }), Full(ColourPiece { variant: Pawn, colour: White }), Full(ColourPiece { variant: Pawn, colour: White }), Full(ColourPiece { variant: Pawn, colour: White }), Full(ColourPiece { variant: Pawn, colour: White }), Full(ColourPiece { variant: Pawn, colour: White }), Full(ColourPiece { variant: Pawn, colour: White }), Full(ColourPiece { variant: Pawn, colour: White }), Full(ColourPiece { variant: Pawn, colour: White }), Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Full(ColourPiece { variant: Pawn, colour: Black }), Full(ColourPiece { variant: Pawn, colour: Black }), Full(ColourPiece { variant: Pawn, colour: Black }), Full(ColourPiece { variant: Pawn, colour: Black }), Full(ColourPiece { variant: Pawn, colour: Black }), Full(ColourPiece { variant: Pawn, colour: Black }), Full(ColourPiece { variant: Pawn, colour: Black }), Full(ColourPiece { variant: Pawn, colour: Black }), Full(ColourPiece { variant: Rook, colour: Black }), Full(ColourPiece { variant: Knight, colour: Black }), Full(ColourPiece { variant: Bishop, colour: Black }), Full(ColourPiece { variant: Queen, colour: Black }), Full(ColourPiece { variant: King, colour: Black }), Full(ColourPiece { variant: Bishop, colour: Black }), Full(ColourPiece { variant: Knight, colour: Black }), Full(ColourPiece { variant: Rook, colour: Black }), ],
            turn: White,

            term_black: Style::new().on(TermColour::RGB(35,35,35)).fg(TermColour::White),
            term_white: Style::new().on(TermColour::RGB(155,155,155)).fg(TermColour::Black),
            term_other: Style::new().fg(TermColour::Green),
        }
    }

    pub fn as_string(&self) -> String {
        let mut board_string = String::with_capacity(128);
        board_string.push_str(&format!("{}", self.term_other.paint("0|")));
        self.pieces.iter().enumerate().for_each(|(index, piece)| {
            if index % 8 == 0 && index != 0 {
                board_string.push_str(&format!("\n{}", self.term_other.paint(format!("{}|", index / 8))))
            } else if index == 0 {}

            let piece_char = match piece {
                Empty => format!("{}", self.term_other.paint(" # ")),
                Full(ColourPiece { variant: Pawn, colour: White }) => format!("{}", self.term_white.paint(" P ")),
                Full(ColourPiece { variant: Pawn, colour: Black }) => format!("{}", self.term_black.paint(" P ")),
                Full(ColourPiece { variant: Knight, colour: White }) => format!("{}", self.term_white.paint(" N ")),
                Full(ColourPiece { variant: Knight, colour: Black }) => format!("{}", self.term_black.paint(" N ")),
                Full(ColourPiece { variant: Bishop, colour: White }) => format!("{}", self.term_white.paint(" B ")),
                Full(ColourPiece { variant: Bishop, colour: Black }) => format!("{}", self.term_black.paint(" B ")),
                Full(ColourPiece { variant: Rook, colour: White }) => format!("{}", self.term_white.paint(" R ")),
                Full(ColourPiece { variant: Rook, colour: Black }) => format!("{}", self.term_black.paint(" R ")),
                Full(ColourPiece { variant: Queen, colour: White }) => format!("{}", self.term_white.paint(" Q ")),
                Full(ColourPiece { variant: Queen, colour: Black }) => format!("{}", self.term_black.paint(" Q ")),
                Full(ColourPiece { variant: King, colour: White }) => format!("{}", self.term_white.paint(" K ")),
                Full(ColourPiece { variant: King, colour: Black }) => format!("{}", self.term_black.paint(" K ")),
            };
            board_string.push_str(&piece_char);

            if (index+1) % 8 == 0 {
                board_string.push_str(&format!("{}", self.term_other.paint(format!("|{}", index / 8))));
            }
        });

        let column_label = self.term_other.paint("   A  B  C  D  E  F  G  H \n");

        // Add letters at top
        board_string.push_str(&format!("\n{}", column_label));

        // Terminal displays top to bottom, but board is bottom to top. So lines must be reversed
        board_string = board_string.lines().rev().map(|line| String::from(line) + "\n").collect();

        //Add letters at bottom
        board_string.push_str(&format!("{}\n", column_label));

        board_string
    }

    pub fn move_piece(&mut self, _move: Move) -> Result<(), String> {
        if !validate_move(_move, self) { return Err("Move was invalid...".to_string()); }

        self.pieces[_move.end.index as usize] = Full(_move.piece);
        self.pieces[_move.start.index as usize] = Empty;
        Ok(())
    }

    // Returns the piece at coord, or none if coord is invalid or square at coord is empty
    pub fn piece_at_coord(&self, coord: &Coord) -> Option<ColourPiece> {
        if validate_coord(coord) {
            match self.pieces[Square::from_coord(coord).index as usize] {
                Full(piece) => Some(piece),
                Empty => None
            }
        } else {
            None
        }
    }
}

fn validate_coord(coord: &Coord) -> bool {
    !(coord.column < 0 || coord.column > 7 || coord.row < 0 || coord.row > 7)
}

fn get_blocked_line(line: &Vec<Coord>, piece_colour: &Colour, board: &Board) -> Vec<Coord> {
    let mut piece_found = false;
    line.iter()
        .filter(|v| validate_coord(*v))
        .take_while(|v| {
            let space = board.pieces[Square::from_coord(*v).index as usize];
            match space {
                Empty => true,
                Full(piece) => {
                    // When find a piece, exclude all new pieces.
                    // Also if piece is of same colour, exclude it, but if piece is opposite then include
                    if piece_found { return false; };
                    piece_found = true;

                    &piece.colour != piece_colour
                }
            }
        })
        .cloned()
        .collect()
}

fn pawn_moves(colour: &Colour, coord: &Coord, board: &Board) -> Vec<Coord> {
    let row = coord.row;
    let column = coord.column;

    let (straight, diagonals) = match colour {
        White => {
            let one_up = Coord { row: row + 1, column };
            let two_up = Coord { row: row + 2, column };
            let north_east = Coord { row: row + 1, column: column + 1 };
            let north_west = Coord { row: row + 1, column: column - 1 };

            if row == 1 {
                (vec![one_up, two_up], vec![north_east, north_west])
            } else {
                (vec![one_up], vec![north_east, north_west])
            }
        }
        Black => {
            let one_down = Coord { row: row - 1, column };
            let two_down = Coord { row: row - 2, column };
            let south_east = Coord { row: row - 1, column: column + 1 };
            let south_west = Coord { row: row - 1, column: column - 1 };

            if row == 6 {
                (vec![one_down, two_down], vec![south_east, south_west])
            } else {
                (vec![one_down], vec![south_east, south_west])
            }
        }
    };

    let opposite_colour = match colour {
        White => Black,
        Black => White
    };

    // Block on both colours, because a pawn cannot take forwards
    let mut straight = get_blocked_line(
        &get_blocked_line(&straight, colour, board),
        &opposite_colour,
        board);

    let mut diagonals: Vec<Coord> = diagonals
        .into_iter()
        .filter(|v| {
            // Only allow diagonals if pieces diagonally are opposite colour
            match board.piece_at_coord(v) {
                Some(piece) => piece.colour != *colour,
                None => false
            }
        })
        .collect();

    let mut moves: Vec<Coord> = Vec::with_capacity(4);
    moves.append(&mut straight);
    moves.append(&mut diagonals);

    moves
}

fn knight_moves(coord: &Coord, colour: &Colour, board: &Board) -> Vec<Coord> {
    let row = coord.row;
    let column = coord.column;

    vec![
        Coord { row: row + 2, column: column + 1 },
        Coord { row: row + 2, column: column - 1 },
        Coord { row: row + 1, column: column + 2 },
        Coord { row: row + 1, column: column - 2 },
        Coord { row: row - 1, column: column + 2 },
        Coord { row: row - 1, column: column - 2 },
        Coord { row: row - 2, column: column + 1 },
        Coord { row: row - 2, column: column - 1 },
    ].into_iter().flat_map(|v| get_blocked_line(&vec![v], colour, board)).collect()
}

fn bishop_moves(coord: &Coord, colour: &Colour, board: &Board) -> Vec<Coord> {
    let row = coord.row;
    let column = coord.column;

    let mut north_east: Vec<Coord> = get_blocked_line(&(1..8).map(|v| Coord { row: row + v, column: column + v }).collect(), colour, board);
    let mut north_west: Vec<Coord> = get_blocked_line(&(1..8).map(|v| Coord { row: row + v, column: column - v }).collect(), colour, board);
    let mut south_east: Vec<Coord> = get_blocked_line(&(1..8).map(|v| Coord { row: row - v, column: column + v }).collect(), colour, board);
    let mut south_west: Vec<Coord> = get_blocked_line(&(1..8).map(|v| Coord { row: row - v, column: column - v }).collect(), colour, board);

    north_east.append(&mut north_west);
    north_east.append(&mut south_east);
    north_east.append(&mut south_west);

    north_east
}

fn rook_moves(coord: &Coord, colour: &Colour, board: &Board) -> Vec<Coord> {
    let row = coord.row;
    let column = coord.column;

    let mut right: Vec<Coord> = get_blocked_line(&(1..8).map(|v| Coord { row, column: column + v }).collect(), colour, board);
    let mut up: Vec<Coord> = get_blocked_line(&(1..8).map(|v| Coord { row: row + v, column }).collect(), colour, board);
    let mut down: Vec<Coord> = get_blocked_line(&(1..8).map(|v| Coord { row: row - v, column }).collect(), colour, board);
    let mut left: Vec<Coord> = get_blocked_line(&(1..8).map(|v| Coord { row, column: column - v }).collect(), colour, board);

    right.append(&mut up);
    right.append(&mut down);
    right.append(&mut left);

    right
}

fn king_moves(coord: &Coord, colour: &Colour, board: &Board) -> Vec<Coord> {
    let row = coord.row;
    let column = coord.column;

    // Copy from rook and bishop
    let mut north_east: Vec<Coord> = get_blocked_line(&(-1..2).map(|v| Coord { row: row + v, column: column + v }).collect(), colour, board);
    let mut north_west: Vec<Coord> = get_blocked_line(&(-1..2).map(|v| Coord { row: row + v, column: column - v }).collect(), colour, board);
    let mut south_east: Vec<Coord> = get_blocked_line(&(-1..2).map(|v| Coord { row: row - v, column: column + v }).collect(), colour, board);
    let mut south_west: Vec<Coord> = get_blocked_line(&(-1..2).map(|v| Coord { row: row - v, column: column - v }).collect(), colour, board);
    north_east.append(&mut north_west);
    north_east.append(&mut south_east);
    north_east.append(&mut south_west);

    let mut right: Vec<Coord> = get_blocked_line(&(-1..2).map(|v| Coord { row, column: column + v }).collect(), colour, board);
    let mut up: Vec<Coord> = get_blocked_line(&(-1..2).map(|v| Coord { row: row + v, column }).collect(), colour, board);
    let mut down: Vec<Coord> = get_blocked_line(&(-1..2).map(|v| Coord { row: row - v, column }).collect(), colour, board);
    let mut left: Vec<Coord> = get_blocked_line(&(-1..2).map(|v| Coord { row, column: column - v }).collect(), colour, board);
    right.append(&mut up);
    right.append(&mut down);
    right.append(&mut left);

    north_east.append(&mut right);

    north_east
}

fn get_piece_moves(piece: ColourPiece, index: i32, board: &Board) -> Vec<Move> {
    if !(0..=63).contains(&index) {
        panic!("Index given was: {}, when max is 63.", index)
    }
    let coord = Square::from_index(index).coord;

    let potential_coords: Vec<Coord> = match piece {
        ColourPiece { variant: Pawn, colour } => pawn_moves(&colour, &coord, board),

        ColourPiece { variant: Knight, colour } => knight_moves(&coord, &colour, board),

        ColourPiece { variant: Bishop, colour } => bishop_moves(&coord, &colour, board),

        ColourPiece { variant: Rook, colour } => rook_moves(&coord, &colour, board),

        ColourPiece { variant: Queen, colour } =>
            bishop_moves(&coord, &colour, board)
                .into_iter()
                .chain(
                    rook_moves(&coord, &colour, board).into_iter()
                ).collect(),

        ColourPiece { variant: King, colour } => king_moves(&coord, &colour, board)
    };

    potential_coords.into_iter()
        .filter(|v| {
            if validate_coord(v) {
                *v != coord && match board.pieces[Square::from_coord(v).index as usize] {
                    Full(colour_piece) => colour_piece.colour != piece.colour,
                    Empty => true,
                }
            } else {
                false
            }
        })
        .map(|v| Move {
            piece,
            start: Square::from_coord(&coord),
            end: Square::from_coord(&v),
        })
        .collect()
}

fn locate_from_target_move(piece: &ColourPiece, desired_square: Square, board: &Board) -> Result<Square, String> {
    let piece_matches: Vec<Square> = board.pieces
        .iter()
        .enumerate()
        .filter(|(index, value)| {
            if let Full(cpiece) = value {
                cpiece.variant == piece.variant && get_piece_moves(*cpiece, (*index) as i32, board)
                    .iter()
                    .map(|v| v.end)
                    .any(|v| v == desired_square)
            } else {
                false
            }
        })
        .map(|(index, ..)| Square::from_index(index as i32))
        .collect();

    match piece_matches.len() {
        0 => Err("No pieces can make that move.".to_string()),
        1 => Ok(piece_matches[0]),
        _ => Err("Try choosing a piece with a specific coordinate. E.g. rb3b5".to_string()),
    }
}

fn validate_move(_move: Move, board: &Board) -> bool {
    let valid_moves = get_piece_moves(_move.piece, _move.start.index, board);
    valid_moves.contains(&_move)
}

pub fn parse_str_move(move_string: &str, board: &Board) -> Result<Move, String> {
    if !(move_string.is_ascii()) {
        return Err(String::from("String passed was not ascii"));
    };

    let char_vec: Vec<char> = move_string.chars().collect();

    match char_vec.len() {
        3 => {
            let piece_type = ColourPiece::from_char(char_vec[0], board).ok_or_else(|| String::from("Invalid piece type"))?;
            let end_square = Square::from_str(&char_vec[1..3])?;

            let start_square = locate_from_target_move(&piece_type, end_square, board)?;

            // Colour on the temporary piece_type variable is unreliable
            if let Full(actual_piece) = board.pieces[start_square.index as usize] {
                let new_move = Move {
                    piece: actual_piece,
                    start: start_square,
                    end: end_square,
                };
                if validate_move(new_move, board) {
                    Ok(new_move)
                } else {
                    Err("Move was invalid".to_string())
                }
            } else {
                panic!("Start square did not have a valid piece on it?")
            }
        }
        5 => {
            let start_square = Square::from_str(&char_vec[1..3])?;
            let end_square = Square::from_str(&char_vec[3..5])?;

            if let Full(actual_piece) = board.pieces[start_square.index as usize] {
                let new_move = Move {
                    piece: actual_piece,
                    start: start_square,
                    end: end_square,
                };
                if validate_move(new_move, board) {
                    Ok(new_move)
                } else {
                    Err("Move was invalid".to_string())
                }
            } else {
                Err("Start square had no piece on it".to_string())
            }
        }
        length => Err(format!("Invalid length passed: {}", length))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Board, Square};

    #[test]
    fn board() {
        let board = Board::new();
        println!("{}", board.as_string());
    }

    #[test]
    fn square() {
        let indexed_square1 = Square::from_index(0).coord;
        assert_eq!(indexed_square1.row, 0);
        assert_eq!(indexed_square1.column, 0);

        let indexed_square2 = Square::from_index(8).coord;
        assert_eq!(indexed_square2.row, 1);
        assert_eq!(indexed_square2.column, 0);

        let indexed_square3 = Square::from_index(5).coord;
        assert_eq!(indexed_square3.row, 0);
        assert_eq!(indexed_square3.column, 5);

        let indexed_square4 = Square::from_index(19).coord;
        assert_eq!(indexed_square4.row, 2);
        assert_eq!(indexed_square4.column, 3)
    }
}

use std::io::Empty;
use ansi_term::{Colour as TermColour, Style};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Colour {
    White,
    Black,
}

use Colour::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Piece {
    Empty,
    // For empty spaces on the board
    Pawn(Colour),
    Knight(Colour),
    Bishop(Colour),
    Rook(Colour),
    Queen(Colour),
    King(Colour),
}

impl Piece {
    fn from_char(char: char, board: &Board) -> Option<Piece> {
        match char {
            'p' => Some(Pawn(board.turn)),
            'b' => Some(Bishop(board.turn)),
            'n' => Some(Knight(board.turn)),
            'r' => Some(Rook(board.turn)),
            'q' => Some(Queen(board.turn)),
            'k' => Some(King(board.turn)),
            _ => None
        }
    }
}

use Piece::*;


#[derive(PartialEq, Copy, Clone, Debug)]
struct Coord {
    row: i32,
    column: i32,
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct Square {
    coord: Coord,
    index: i32,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Move {
    piece: Piece,
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
        if coord.column < 0 || coord.column > 63 || coord.row < 0 || coord.row > 63 {
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
    pub pieces: [Piece; 64],
    pub turn: Colour,

    term_white: Style,
    term_black: Style,
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
            turn: White,

            term_white: Style::new().on(TermColour::Black).fg(TermColour::White),
            term_black: Style::new().on(TermColour::White).fg(TermColour::Black),
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

    pub fn move_piece(&self, _move: Move) -> Result<(), String> {
        println!("{:#?}", _move);
        Ok(())
    }
}

fn validate_coord(coord: &Coord) -> bool {
    !(coord.column < 0 || coord.column > 63 || coord.row < 0 || coord.row > 63)
}

fn get_blocked_line(line: &Vec<Coord>, board: &Board) -> Vec<Coord> {
    line.iter()
        .filter(|v| validate_coord(*v))
        .take_while(|v| board.pieces[Square::from_coord(*v).index as usize] == Empty )
        .cloned()
        .collect()
}

fn pawn_moves(colour: &Colour, coord: &Coord, board: &Board) -> Vec<Coord> {
    let row = coord.row;
    let column = coord.column;

    match colour {
        White => {
            let mut moves = Vec::with_capacity(2);
            let one_up = Coord { row: row + 1, column };
            moves.push(one_up);
            if row == 1 {
                moves.push(Coord { row: row + 2, column });
            }

            get_blocked_line(&moves, board)
        }
        Black => {
            let mut moves = Vec::with_capacity(2);
            let one_down = Coord { row: row - 1, column };
            moves.push(one_down);
            if row == 6  {
                moves.push(Coord { row: row - 2, column });
            }

            get_blocked_line(&moves, board)
        }
    }
}

fn knight_moves(coord: &Coord) -> Vec<Coord> {
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
    ]
}

fn bishop_moves(coord: &Coord, board: &Board) -> Vec<Coord> {
    let row = coord.row;
    let column = coord.column;

    let mut north_east: Vec<Coord> = get_blocked_line(&(1..8).map(|v| Coord {row: row + v, column: column + v}).collect(), board);
    let mut north_west: Vec<Coord> = get_blocked_line(&(1..8).map(|v| Coord {row: row + v, column: column - v}).collect(), board);
    let mut south_east: Vec<Coord> = get_blocked_line(&(1..8).map(|v| Coord {row: row - v, column: column + v}).collect(), board);
    let mut south_west: Vec<Coord> = get_blocked_line(&(1..8).map(|v| Coord {row: row - v, column: column - v}).collect(), board);

    north_east.append(&mut north_west);
    north_east.append(&mut south_east);
    north_east.append(&mut south_west);

    north_east
}

fn rook_moves(coord: &Coord, board: &Board) -> Vec<Coord> {
    let row = coord.row;
    let column = coord.column;

    let mut right: Vec<Coord> = get_blocked_line(&(1..8).map(|v| Coord {row, column: column + v}).collect(), board);
    let mut up: Vec<Coord> = get_blocked_line(&(1..8).map(|v| Coord {row: row + v, column}).collect(), board);
    let mut down: Vec<Coord> = get_blocked_line(&(1..8).map(|v| Coord {row: row - v, column}).collect(), board);
    let mut left: Vec<Coord> = get_blocked_line(&(1..8).map(|v| Coord {row, column: column - v}).collect(), board);

    right.append(&mut up);
    right.append(&mut down);
    right.append(&mut left);

    right
}

fn king_moves(coord: &Coord) -> Vec<Coord> {
    let row = coord.row;
    let column = coord.column;

    (-1..2).flat_map(|val| vec![
        // Combine the rook and bishop moves
        Coord { row: row + val, column },
        Coord { row, column: column + val },
        Coord { row: row + val, column: column + val },
        Coord { row: row + val, column: column - val },
    ]).collect()
}

fn get_piece_moves(piece: &Piece, index: i32, board: &Board) -> Vec<Move> {
    if !(0..=63).contains(&index) {
        panic!("Index given was: {}, when max is 63.", index)
    }
    let coord = Square::from_index(index).coord;

    let potential_coords: Vec<Coord> = match piece {
        Empty => vec![],
        // Todo: pawn diagonals
        Pawn(colour) => pawn_moves(colour, &coord, board),
        Knight(_) => knight_moves(&coord),
        Bishop(_) => bishop_moves(&coord, board),
        Rook(_) => rook_moves(&coord, board),
        Queen(_) =>
            bishop_moves(&coord, board)
                .into_iter()
                .chain(
                    knight_moves(&coord).into_iter()
                ).collect(),
        King(_) => king_moves(&coord)
    };

    potential_coords.into_iter()
        .filter(|v| validate_coord(v) && *v != coord)
        .map(|v| Move {
            piece: *piece,
            start: Square::from_coord(&coord),
            end: Square::from_coord(&v),
        })
        .collect()
}

fn locate_from_target_move(piece: &Piece, desired_square: Square, board: &Board) -> Option<Square> {
    let piece_matches: Vec<Square> = board.pieces
        .iter()
        .enumerate()
        .filter(|(index, value)| {
            *value == piece
                && get_piece_moves(value, (*index) as i32, board)
                .iter()
                .map(|v| v.end)
                .any(|v| v == desired_square)
        })
        .map(|(index, value)| Square::from_index(index as i32))
        .collect();

    match piece_matches.len() {
        1 => Some(piece_matches[0]),
        _ => None
    }
}

fn validate_move(_move: Move) -> bool {
    let valid_moves = get_piece_moves(&_move.piece, _move.start.index);
    println!("{:#?}", valid_moves);
    valid_moves.contains(&_move)
}

pub fn parse_str_move(move_string: &str, board: &Board) -> Result<Move, String> {
    if !(move_string.is_ascii()) {
        return Err(String::from("String passed was not ascii"));
    };

    let char_vec: Vec<char> = move_string.chars().collect();

    match char_vec.len() {
        3 => {
            let piece_type = Piece::from_char(char_vec[0], board).ok_or_else(|| String::from("Invalid piece type"))?;
            let end_square = Square::from_str(&char_vec[1..3])?;

            println!("{:#?}", piece_type);
            println!("{:#?}", end_square);

            let start_square = locate_from_target_move(&piece_type, end_square, board)
                .ok_or_else(|| String::from("Could not evaluate move, try specifying a starting square. E.g. rb3b5"))?;

            let new_move = Move {
                piece: piece_type,
                start: start_square,
                end: end_square
            };
            if validate_move(new_move) {
                Ok(new_move)
            } else {
                Err("Move was invalid".to_string())
            }
        }
        5 => {
            let piece_type = Piece::from_char(char_vec[0], board).ok_or_else(|| String::from("Invalid piece type"))?;
            let start_square = Square::from_str(&char_vec[1..3])?;
            let end_square = Square::from_str(&char_vec[3..5])?;
            let new_move = Move {
                piece: piece_type,
                start: start_square,
                end: end_square
            };

            println!("{:#?}", new_move);

            if validate_move(new_move) {
                Ok(new_move)
            } else {
                Err("Move was invalid".to_string())
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

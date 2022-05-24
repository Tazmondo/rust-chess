mod moves;

use Colour::*;
use Piece::*;
use Space::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Colour {
    White,
    Black,
}

impl std::ops::Not for Colour {
    type Output = Colour;

    fn not(self) -> Self::Output {
        match self {
            White => Black,
            Black => White
        }
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CastleSide {
    King,
    Queen,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ColourPiece {
    pub variant: Piece,
    pub colour: Colour,
}

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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Space {
    Empty,
    Full(ColourPiece),
}

// Represents an arbitrary coordinate
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Coord {
    pub row: i32,
    pub column: i32,
}

// Represents a coordinate that actually exists on the board
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Square {
    pub coord: Coord,
    pub index: i32,
}

impl Square {
    pub fn from_index(index: i32) -> Square {
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

    pub fn from_coord(coord: &Coord) -> Square {
        if coord.column < 0 || coord.column > 7 || coord.row < 0 || coord.row > 7 {
            panic!("Invalid row and/or column passed:\nrow: {}\ncolumn: {}", coord.row, coord.column)
        }

        Square {
            index: (coord.row * 8) + coord.column,
            coord: *coord,
        }
    }

    pub fn from_str(coords: &[char]) -> Result<Square, String> {
        let column = match &coords[0] {
            'a' | '1' => Ok(0),
            'b' | '2' => Ok(1),
            'c' | '3' => Ok(2),
            'd' | '4' => Ok(3),
            'e' | '5' => Ok(4),
            'f' | '6' => Ok(5),
            'g' | '7' => Ok(6),
            'h' | '8' => Ok(7),
            _ => Err("Invalid column letter")
        }?;
        let row = coords[1].to_digit(10).ok_or("Row was not a number")? as i32 - 1;
        if !(0..=7).contains(&row) {
            return Err(String::from("Row was not 1 to 8"));
        }

        let index = (row * 8) + column;

        Ok(Square::from_index(index))
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Move {
    piece: ColourPiece,
    start: Square,
    end: Square,
}

impl Move {
    fn inverse(&self) -> Move {
        Move {
            piece: self.piece,
            start: self.end,
            end: self.start,
        }
    }
}

pub enum GameState {
    Playing,
    Checkmate(Colour),
    // Colour that has been checkmated
    Stalemate,
}

// Board indexes will start at bottom left.
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Board {
    pub pieces: [Space; 64],
    pub turn: Colour,
    can_white_castle: [Option<CastleSide>; 2],
    can_black_castle: [Option<CastleSide>; 2],
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
            pieces: [Full(ColourPiece { variant: Rook, colour: Black }), Full(ColourPiece { variant: Knight, colour: Black }), Full(ColourPiece { variant: Bishop, colour: Black }), Full(ColourPiece { variant: Queen, colour: Black }), Full(ColourPiece { variant: King, colour: Black }), Full(ColourPiece { variant: Bishop, colour: Black }), Full(ColourPiece { variant: Knight, colour: Black }), Full(ColourPiece { variant: Rook, colour: Black }), Full(ColourPiece { variant: Pawn, colour: Black }), Full(ColourPiece { variant: Pawn, colour: Black }), Full(ColourPiece { variant: Pawn, colour: Black }), Full(ColourPiece { variant: Pawn, colour: Black }), Full(ColourPiece { variant: Pawn, colour: Black }), Full(ColourPiece { variant: Pawn, colour: Black }), Full(ColourPiece { variant: Pawn, colour: Black }), Full(ColourPiece { variant: Pawn, colour: Black }), Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Full(ColourPiece { variant: Pawn, colour: White }), Full(ColourPiece { variant: Pawn, colour: White }), Full(ColourPiece { variant: Pawn, colour: White }), Full(ColourPiece { variant: Pawn, colour: White }), Full(ColourPiece { variant: Pawn, colour: White }), Full(ColourPiece { variant: Pawn, colour: White }), Full(ColourPiece { variant: Pawn, colour: White }), Full(ColourPiece { variant: Pawn, colour: White }), Full(ColourPiece { variant: Rook, colour: White }), Full(ColourPiece { variant: Knight, colour: White }), Full(ColourPiece { variant: Bishop, colour: White }), Full(ColourPiece { variant: Queen, colour: White }), Full(ColourPiece { variant: King, colour: White }), Full(ColourPiece { variant: Bishop, colour: White }), Full(ColourPiece { variant: Knight, colour: White }), Full(ColourPiece { variant: Rook, colour: White }), ],
            turn: White,
            can_black_castle: [Some(CastleSide::Queen), Some(CastleSide::King)],
            can_white_castle: [Some(CastleSide::Queen), Some(CastleSide::King)],
        }
    }

    // No checks, this is called when checking for check, as using the move_piece function resulted
    // in infinite recursion and a stack overflow
    fn execute_move(&mut self, _move: Move) {
        self.pieces[_move.end.index as usize] = Full(_move.piece);
        self.pieces[_move.start.index as usize] = Empty;

        // Swap rook to new position during castle
        if _move.piece.variant == King && _move.start.coord.column == 4 && (_move.end.coord.column == 2 || _move.end.coord.column == 6) {
            let (old_rook, new_rook) = match _move.end.coord {
                Coord { row, column: 6 } => {
                    (Square::from_coord(&Coord { row, column: 7 }).index, Square::from_coord(&Coord { row, column: 5 }).index)
                }
                Coord { row, column: 2 } => {
                    (Square::from_coord(&Coord { row, column: 0 }).index, Square::from_coord(&Coord { row, column: 3 }).index)
                }
                _ => {
                    panic!("Unexpected rook position for castle: {:?}", _move)
                }
            };
            self.pieces[new_rook as usize] = self.pieces[old_rook as usize];
            self.pieces[old_rook as usize] = Empty;
        };
    }

    pub fn move_piece(&mut self, mut _move: Move) -> Result<GameState, String> {
        if !self.validate_move(_move) { return Err("move_piece: Move was invalid...".to_string()); }
        if self.turn != _move.piece.colour {
            return Err(format!("It is currently {:?}'s turn!", self.turn));
        };

        // Promotion   todo: allow for rook bishop and knight option
        // Works because move functions dont check that
        // the piece at start of move is the piece in move.piece
        match _move.piece {
            ColourPiece { variant: Pawn, colour: White } if _move.end.coord.row == 0 => {
                _move.piece = ColourPiece {
                    variant: Queen,
                    colour: White,
                }
            }
            ColourPiece { variant: Pawn, colour: Black } if _move.end.coord.row == 7 => {
                _move.piece = ColourPiece {
                    variant: Queen,
                    colour: Black,
                }
            }
            _ => {}
        }

        let check = self.does_move_cause_check(_move);
        match check {
            Some(White) => return Err("White would be in check!".to_string()),
            Some(Black) => return Err("Black would be in check!".to_string()),
            _ => {}
        }

        // Disable castling upon moving a rook/king
        if _move.piece.variant == Rook && _move.start.coord.row == 0 {
            let side = match _move.start.coord.column {
                0 => Some(CastleSide::Queen),
                7 => Some(CastleSide::King),
                _ => None
            };
            if side.is_some() {
                match (self.turn, side.unwrap()) {
                    (White, CastleSide::Queen) => { self.can_white_castle[0] = None }
                    (White, CastleSide::King) => { self.can_white_castle[1] = None }
                    (Black, CastleSide::Queen) => { self.can_black_castle[0] = None }
                    (Black, CastleSide::King) => { self.can_black_castle[1] = None }
                }
            }
        } else if _move.piece.variant == King {
            match self.turn {
                White => self.can_white_castle = [None, None],
                Black => self.can_black_castle = [None, None],
            };
        };

        self.execute_move(_move);

        // Switch to perspective of opposing player
        self.turn = !self.turn;

        // Check for checkmate before returning control to the player
        Ok(self.check_mate())
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

    // Must be an option, as when checking for threats, it assumes that the piece will take the king
    // resulting in a board with no king when checking.
    fn get_king(&self, colour: &Colour) -> Option<Square> {
        self.pieces.iter().enumerate().find_map(|(index, space)| {
            match space {
                Empty => None,
                Full(piece) => {
                    if &piece.colour == colour && piece.variant == King {
                        Some(Square::from_index(index as i32))
                    } else { None }
                }
            }
        })
    }

    fn get_possible_moves(&self, colour: &Colour, exclude_castle: bool) -> Vec<Move> {
        self.pieces.iter()
            .enumerate()
            .filter_map(|(index, v)| match v {
                Empty => None,
                Full(piece) => {
                    if &piece.colour == colour {
                        Some(self.get_piece_moves(*piece, index as i32, exclude_castle))
                    } else {
                        None
                    }
                }
            })
            .flatten()
            .collect()
    }

    fn is_threatened(&self, threatening_colour: &Colour, square: Square) -> bool {
        let vec = self.get_possible_moves(threatening_colour, true);
        vec.iter().any(|v| v.end == square)
    }

    fn in_check_state(&self) -> Option<Colour> {
        let w_king = match self.get_king(&White) {
            Some(square) => square,
            None => return None
        };

        let b_king = match self.get_king(&Black) {
            Some(square) => square,
            None => return None
        };

        if self.is_threatened(&Black, w_king) && self.turn == White {
            Some(White)
        } else if self.is_threatened(&White, b_king) && self.turn == Black {
            Some(Black)
        } else {
            None
        }
    }

    fn check_mate(&self) -> GameState {
        let moves: Vec<Move> = self.get_possible_moves(&self.turn, false)
            .iter()
            .filter(|v| match self.does_move_cause_check(**v) {
                None => true,
                Some(colour) => { colour != self.turn }
            })
            .copied()
            .collect();

        let no_moves = moves.is_empty();

        if no_moves && self.in_check_state().is_some() {
            GameState::Checkmate(self.turn)
        } else if no_moves {
            GameState::Stalemate
        } else {
            GameState::Playing
        }
    }

    fn does_move_cause_check(&self, _move: Move) -> Option<Colour> {
        let mut new_board = *self;
        new_board.execute_move(_move);
        new_board.in_check_state()
    }

    pub fn attempt_move_with_coords(&mut self, start: Coord, end: Coord) -> Result<GameState, String> {
        if validate_coord(&start) && validate_coord(&end) {
            let start = Square::from_coord(&start);
            let end = Square::from_coord(&end);
            let piece = self.piece_at_coord(&start.coord).ok_or_else(|| "Empty square used as start.".to_string())?;

            let new_move = Move { piece, start, end };

            if self.validate_move(new_move) {
                self.move_piece(new_move)
            } else {
                Err("attempt_move: Move was invalid".to_string())
            }
        } else {
            Err("attempt_move: Invalid coords passed".to_string())
        }
    }

    fn locate_from_target_move(&self, piece: &ColourPiece, desired_square: Square) -> Result<Square, String> {
        let piece_matches: Vec<Square> = self.pieces
            .iter()
            .enumerate()
            .filter(|(index, value)| {
                if let Full(cpiece) = value {
                    cpiece.variant == piece.variant
                        && cpiece.colour == piece.colour
                        && self.get_piece_moves(*cpiece, (*index) as i32, false)
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

    pub fn get_piece_moves(&self, piece: ColourPiece, index: i32, exclude_castle: bool) -> Vec<Move> {
        if !(0..=63).contains(&index) {
            panic!("Index given was: {}, when max is 63.", index)
        }
        let coord = Square::from_index(index).coord;

        let potential_coords: Vec<Coord> = match piece.variant {
            King => moves::piece_moveset(piece, self, &coord, exclude_castle),
            _ => moves::piece_moveset(piece, self, &coord, false)
        };

        potential_coords.into_iter()
            .filter(|v| {
                if validate_coord(v) {
                    *v != coord && match self.piece_at_coord(v) {
                        Some(colour_piece) => colour_piece.colour != piece.colour,
                        None => true,
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

    fn validate_move(&self, _move: Move) -> bool {
        let valid_moves = self.get_piece_moves(_move.piece, _move.start.index, false);

        valid_moves.contains(&_move)
    }

    pub fn can_square_move(&self, space: &Space, square: &Square) -> bool {
        match space {
            Empty => false,
            Full(piece) => !self.get_piece_moves(*piece, square.index, false).is_empty() && piece.colour == self.turn
        }
    }

    fn can_castle(&self, colour: &Colour, side: CastleSide) -> bool {
        match colour {
            White => {
                if !self.can_white_castle.contains(&Some(side)) {
                    return false;
                }
            }
            Black => {
                if !self.can_black_castle.contains(&Some(side)) {
                    return false;
                }
            }
        }

        // Last square of check squares is new position of rook
        // Second last square of check squares is new position of king
        let check_squares = match colour {
            Black => {
                match side {
                    CastleSide::King => {
                        vec![
                            Square::from_index(5),
                            Square::from_index(6),
                        ]
                    }
                    CastleSide::Queen => {
                        vec![
                            Square::from_index(1),
                            Square::from_index(2),
                            Square::from_index(3),
                        ]
                    }
                }
            }
            White => {
                match side {
                    CastleSide::King => {
                        vec![
                            Square::from_coord(&Coord { row: 7, column: 6 }),
                            Square::from_coord(&Coord { row: 7, column: 5 }),
                        ]
                    }
                    CastleSide::Queen => {
                        vec![
                            Square::from_coord(&Coord { row: 7, column: 1 }),
                            Square::from_coord(&Coord { row: 7, column: 2 }),
                            Square::from_coord(&Coord { row: 7, column: 3 }),
                        ]
                    }
                }
            }
        };

        // Make sure none of the squares are threatened
        !check_squares.iter().any(|v| {
            let threatened = self.is_threatened(&(!*colour), *v) || matches!(self.pieces[v.index as usize], Full(_piece));
            threatened
        })
    }
}

fn validate_coord(coord: &Coord) -> bool {
    !(coord.column < 0 || coord.column > 7 || coord.row < 0 || coord.row > 7)
}


pub fn parse_str_move(move_string: &str, board: &Board) -> Result<Move, String> {
    if !(move_string.is_ascii()) {
        return Err(String::from("String passed was not ascii"));
    };

    let char_vec: Vec<char> = move_string.chars().collect();

    // Todo: castling
    match char_vec.len() {
        3 => {
            let piece_type = ColourPiece::from_char(char_vec[0], board).ok_or_else(|| String::from("Invalid piece type"))?;
            let end_square = Square::from_str(&char_vec[1..3])?;

            let start_square = board.locate_from_target_move(&piece_type, end_square)?;

            // Colour on the temporary piece_type variable is unreliable
            if let Full(actual_piece) = board.pieces[start_square.index as usize] {
                let new_move = Move {
                    piece: actual_piece,
                    start: start_square,
                    end: end_square,
                };
                if board.validate_move(new_move) {
                    Ok(new_move)
                } else {
                    Err("parse_str_move: Move was invalid".to_string())
                }
            } else {
                panic!("Start square did not have a valid piece on it?")
            }
        }
        // 4 => {
        //     let start_square = Square::from_str(&char_vec[..2])?;
        //     let end_square = Square::from_str(&char_vec[2..4])?;
        //     let piece = match board.piece_at_coord(&start_square.coord) {
        //         None => return Err("Start square did not contain a piece.".to_string()),
        //         Some(piece) => piece
        //     };
        //
        //     let new_move = Move {
        //         piece,
        //         start: start_square,
        //         end: end_square
        //     };
        //
        //     if validate_move(new_move, board) {
        //         Ok(new_move)
        //     } else {
        //         Err("Move was invalid".to_string())
        //     }
        // }
        5 => {
            let start_square = Square::from_str(&char_vec[1..3])?;
            let end_square = Square::from_str(&char_vec[3..5])?;

            if let Full(actual_piece) = board.pieces[start_square.index as usize] {
                let new_move = Move {
                    piece: actual_piece,
                    start: start_square,
                    end: end_square,
                };
                if board.validate_move(new_move) {
                    Ok(new_move)
                } else {
                    Err("parse_str: Move was invalid".to_string())
                }
            } else {
                Err("Start square had no piece on it".to_string())
            }
        }

        any_length => Err(format!("Invalid length passed: {}", any_length))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Black, Board, Square, White};

    #[test]
    fn board() {
        let board = Board::new();
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

    #[test]
    fn colour() {
        assert_eq!(White, !Black);
        assert_eq!(White, White);
        assert_eq!(Black, !White);
        assert_eq!(Black, Black);
        assert_ne!(Black, White);
    }
}

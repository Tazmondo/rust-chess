use crate::*;

pub fn piece_moveset(piece: ColourPiece, board: &Board, coord: &Coord, exclude_castle: bool) -> Vec<Coord> {
    match piece {
        ColourPiece { variant: Pawn, colour } => pawn_moves(&colour, coord, board),

        ColourPiece { variant: Knight, colour } => knight_moves(coord, &colour, board),

        ColourPiece { variant: Bishop, colour } => bishop_moves(coord, &colour, board),

        ColourPiece { variant: Rook, colour } => rook_moves(coord, &colour, board),

        ColourPiece { variant: Queen, colour } =>
            bishop_moves(coord, &colour, board)
                .into_iter()
                .chain(
                    rook_moves(coord, &colour, board).into_iter()
                ).collect(),

        ColourPiece { variant: King, colour } => king_moves(coord, &colour, board, exclude_castle)
    }
}

fn get_blocked_line(line: &Vec<Coord>, piece_colour: &Colour, board: &Board) -> Vec<Coord> {
    let mut piece_found = false;
    line.iter()
        .filter(|v| validate_coord(*v))
        .take_while(|v| {
            match board.piece_at_coord(v) {
                None => true,
                Some(piece) => {
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
            let one_up = Coord { row: row - 1, column };
            let two_up = Coord { row: row - 2, column };
            let north_east = Coord { row: row - 1, column: column + 1 };
            let north_west = Coord { row: row - 1, column: column - 1 };

            if row == 6 {
                (vec![one_up, two_up], vec![north_east, north_west])
            } else {
                (vec![one_up], vec![north_east, north_west])
            }
        }
        Black => {
            let one_down = Coord { row: row + 1, column };
            let two_down = Coord { row: row + 2, column };
            let south_east = Coord { row: row + 1, column: column + 1 };
            let south_west = Coord { row: row + 1, column: column - 1 };

            if row == 1 {
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

fn king_moves(coord: &Coord, colour: &Colour, board: &Board, exclude_castle: bool) -> Vec<Coord> {
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

    if !exclude_castle {
        let mut castles = Vec::new();

        match colour {
            White => {
                if board.can_castle(colour, CastleSide::King) {
                    castles.push(Coord {row: 7, column: 6})
                }
                if board.can_castle(colour, CastleSide::Queen) {
                    castles.push(Coord {row: 7, column: 2})
                }
            }
            Black => {
                if board.can_castle(colour, CastleSide::King) {
                    castles.push(Coord {row: 0, column: 6})
                }
                if board.can_castle(colour, CastleSide::Queen) {
                    castles.push(Coord {row: 0, column: 2})
                }
            }
        };

        north_east.append(&mut castles);
    }

    north_east
}

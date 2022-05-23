default = "1rnbqkbnrppppppppeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee0pppppppprnbqkbnr"

def make_rust_board(inp):
    colour = "White"
    result = "["
    for char in inp:
        if char == "r":
            result += f"Full(ColourPiece {'{'}variant: Rook, colour:{colour}{'}'}), "
        elif char == "n":
            result += f"Full(ColourPiece {'{'}variant: Knight, colour:{colour}{'}'}), "
        elif char == "b":
            result += f"Full(ColourPiece {'{'}variant: Bishop, colour:{colour}{'}'}), "
        elif char == "q":
            result += f"Full(ColourPiece {'{'}variant: Queen, colour:{colour}{'}'}), "
        elif char == "k":
            result += f"Full(ColourPiece {'{'}variant: King, colour:{colour}{'}'}), "
        elif char == "p":
            result += f"Full(ColourPiece {'{'}variant: Pawn, colour:{colour}{'}'}), "
        elif char == "e":
            result += "Empty, "
        elif char == "0":
            colour = "White"
        elif char == "1":
            colour = "Black"

    result += "],"
    return result


with open("board.txt", "w") as f:
    f.write(make_rust_board(default))

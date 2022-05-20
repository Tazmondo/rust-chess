default = "0rnbqkbnrppppppppeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee1pppppppprnbqkbnr"

def make_rust_board(inp):
    colour = "White"
    result = "["
    for char in inp:
        if char == "r":
            result += f"Rook({colour}), "
        elif char == "n":
            result += f"Knight({colour}), "
        elif char == "b":
            result += f"Bishop({colour}), "
        elif char == "q":
            result += f"Queen({colour}), "
        elif char == "k":
            result += f"King({colour}), "
        elif char == "p":
            result += f"Pawn({colour}), "
        elif char == "e":
            result += "Empty, "
        elif char == "0":
            colour = "White"
        elif char == "1":
            colour = "Black"

    result += "]"
    return result

with open("board.txt", "w") as f:
    f.write(make_rust_board(default))

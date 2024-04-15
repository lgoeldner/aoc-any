from dataclasses import dataclass
from typing import Any
from functools import reduce

TEST = True


@dataclass
class Tree:
    height: int
    visible: bool

    def __repr__(self) -> str:
        return f"[{self.height} {'#' if self.visible else '.'}]"


def main(indata: str):
    data = parse(indata)

    def visible_row(row: list[Tree], repeat: bool = False) -> list[Tree]:
        max = 0
        last = len(row) - 1
        for i, tree in enumerate(row):
            if i == 0 or i == last:
                tree.visible = True
                continue
            if tree.height > max:
                tree.visible = True
                max = tree.height
        if not repeat:
            return visible_row(row, repeat=True)
        return row
    
    def set_row_visible(row):
        for item in row:
            item.visible = True
            
    print(type(data))
    print_lltree(data)
    print(type(data))
    data = [visible_row(row) for row in data]
    set_row_visible(data[0])
    last = len(data) - 1
    set_row_visible(data[last])
    print(type(data))
    print(data)
    #print_lltree(data)
    print(len([item for row in data for item in row if item.visible]))
    
    newmat = [row[1:-1] for row in data[1:-1]]
    print_lltree(newmat)
    
        
def print_lltree(inp: list[list[Tree]]) -> None:
    def print_ltree(inp: list[Tree]):
        print(" ".join([repr(i) for i in inp]))
    [print_ltree(row) for row in inp]
            

def parse(inp: str) -> list[list[Tree]]:
    return [
        [Tree(height=int(height), visible=False) for height in line]
        for line in inp.splitlines()
    ]


def get_test():
    return """30373
25512
65932
33549
35990
"""


if __name__ == "__main__":
    with open("inputs/day8-inp.txt") as f:
        inp = f.read()
    main(get_test() if TEST else inp)

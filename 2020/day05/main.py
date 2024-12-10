#!/usr/bin/env python3

with open("input") as f:
    ROWS = f.read().splitlines()


def main():
    ids = []
    for row in ROWS:
        low_row = 0
        high_row = 127
        low_column = 0
        high_column = 7
        for letter in row:
            if letter == "F":
                diff = (high_row - low_row) // 2
                high_row = low_row + diff
            elif letter == "B":
                diff = (high_row - low_row) // 2
                low_row = low_row + diff + 1
            elif letter == "L":
                diff = (high_column - low_column) // 2
                high_column = low_column + diff
            elif letter == "R":
                diff = (high_column - low_column) // 2
                low_column = low_column + diff + 1
        ids.append((high_row * 8) + high_column)
        ids.sort()
    for n, id_ in enumerate(ids[1:-1]):
        # print(n, id_)
        if ids[n + 2] - id_ != 1:
            print(id_)


if __name__ == '__main__':
    main()

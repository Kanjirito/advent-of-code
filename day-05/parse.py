#!/usr/bin/env python3
import re


def main():
    with open("input", "r") as f:
        file = [line.rstrip("\n") for line in f.readlines()]

    stacks = handle_stacks(file)
    moves = handle_moves(file)

    with open("parsed", "w") as f:
        for stack in stacks:
            f.write(f"{stack}\n")
        f.write("\n")
        for move in moves:
            f.write(f"{move}\n")


def handle_stacks(file):
    stacks = [[] for _ in range(9)]
    for line in file[7::-1]:
        j = 0
        for i in range(1, len(line), 4):
            if line[i] != " ":
                stacks[j].append(line[i])
            j += 1
    return ["".join(stack) for stack in stacks]


def handle_moves(file):
    reg = re.compile(r"move (\d+) from (\d) to (\d)")
    moves = []
    for line in file[10:]:
        result = reg.match(line)
        moves.append(f"{result.group(1)} {result.group(2)} {result.group(3)}")
    return moves


if __name__ == "__main__":
    main()

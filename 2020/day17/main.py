#!/usr/bin/env python3
from itertools import product

# x, y, z, w
ACTIVE = set()


def main():
    load_input()
    part_two()
    print(len(ACTIVE))


def part_one():
    global ACTIVE
    for cycle in range(6):
        new_active = set()
        x_list = []
        y_list = []
        z_list = []
        for node in ACTIVE:
            x_list.append(node[0])
            y_list.append(node[1])
            z_list.append(node[2])
        for (x, y, z) in product(range(min(x_list) - 1, max(x_list) + 2),
                                 range(min(y_list) - 1, max(y_list) + 2),
                                 range(min(z_list) - 1, max(z_list) + 2)):
            active_neighbours = 0
            for neighbour in product(range(x - 1, x + 2),
                                     range(y - 1, y + 2),
                                     range(z - 1, z + 2)):
                if (x, y, z) == neighbour:
                    continue
                elif neighbour in ACTIVE:
                    active_neighbours += 1
            is_active = (x, y, z) in ACTIVE
            if is_active and (active_neighbours == 2 or active_neighbours == 3):
                new_active.add((x, y, z))
            if not is_active and active_neighbours == 3:
                new_active.add((x, y, z))
        ACTIVE = new_active


def part_two():
    global ACTIVE
    for cycle in range(6):
        new_active = set()
        x_list = []
        y_list = []
        z_list = []
        w_list = []
        for node in ACTIVE:
            x_list.append(node[0])
            y_list.append(node[1])
            z_list.append(node[2])
            w_list.append(node[3])
        for (x, y, z, w) in product(range(min(x_list) - 1, max(x_list) + 2),
                                    range(min(y_list) - 1, max(y_list) + 2),
                                    range(min(z_list) - 1, max(z_list) + 2),
                                    range(min(w_list) - 1, max(w_list) + 2)):
            active_neighbours = 0
            for neighbour in product(range(x - 1, x + 2),
                                     range(y - 1, y + 2),
                                     range(z - 1, z + 2),
                                     range(w - 1, w + 2)):
                if (x, y, z, w) == neighbour:
                    continue
                elif neighbour in ACTIVE:
                    active_neighbours += 1
            is_active = (x, y, z, w) in ACTIVE
            if is_active and (active_neighbours == 2 or active_neighbours == 3):
                new_active.add((x, y, z, w))
            if not is_active and active_neighbours == 3:
                new_active.add((x, y, z, w))
        ACTIVE = new_active


def load_input():
    global ACTIVE
    with open("input") as f:
        for y, line in enumerate(f.readlines()):
            for x, node in enumerate(line.strip()):
                if node == "#":
                    ACTIVE.add((x, y, 0, 0))


if __name__ == '__main__':
    main()

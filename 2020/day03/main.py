#!/usr/bin/env python3

with open("input") as f:
    ROWS = f.read().splitlines()


def main():
    tree_count_1 = drive_slope(1, 1)
    tree_count_2 = drive_slope(3, 1)
    tree_count_3 = drive_slope(5, 1)
    tree_count_4 = drive_slope(7, 1)
    tree_count_5 = drive_slope(1, 2)
    print(tree_count_1 * tree_count_2 * tree_count_3 * tree_count_4 * tree_count_5)


def drive_slope(y_count, x_count):
    x_pos = 0
    y_pos = 0
    tree_count = 0
    for n, line in enumerate(ROWS):
        if n % x_count:
            continue
        if line[y_pos] == "#":
            tree_count += 1
        y_pos = ((y_pos + y_count) % 31)
    return tree_count


if __name__ == '__main__':
    main()

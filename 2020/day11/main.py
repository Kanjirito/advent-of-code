#!/usr/bin/env python3
from copy import deepcopy


def main():
    inp = load_input()
    # print(part_one(inp))
    print(part_two(inp))


def part_one(inp):
    old_seats = deepcopy(inp)
    new_seats = deepcopy(old_seats)
    while True:
        for y, row in enumerate(old_seats):
            for x, seat in enumerate(row):
                if seat == ".":
                    continue
                occupied = check_neighbours(old_seats, x, y)
                if seat == "L" and occupied == 0:
                    new_seats[y][x] = "#"
                elif seat == "#" and occupied >= 4:
                    new_seats[y][x] = "L"
        if old_seats == new_seats:
            count = 0
            for row in new_seats:
                for seat in row:
                    if seat == "#":
                        count += 1
            return count
        old_seats = deepcopy(new_seats)


def check_neighbours(seats, x, y):
    seats_len_y = len(seats) - 1
    seats_len_x = len(seats[0]) - 1
    occupied_seats = 0
    for check_y in range(y - 1, y + 2):
        if check_y < 0 or check_y > seats_len_y:
            continue
        for check_x in range(x - 1, x + 2):
            if check_x < 0 or check_x > seats_len_x or (check_x == x and check_y == y):
                continue
            if seats[check_y][check_x] == "#":
                occupied_seats += 1
    return occupied_seats


def part_two(inp):
    old_seats = deepcopy(inp)
    new_seats = deepcopy(old_seats)
    while True:
        for y, row in enumerate(old_seats):
            for x, seat in enumerate(row):
                if seat == ".":
                    continue
                occupied = look_for_seats(old_seats, x, y)
                if seat == "L" and occupied == 0:
                    new_seats[y][x] = "#"
                elif seat == "#" and occupied >= 5:
                    new_seats[y][x] = "L"
        if old_seats == new_seats:
            count = 0
            for row in new_seats:
                for seat in row:
                    if seat == "#":
                        count += 1
            return count
        old_seats = deepcopy(new_seats)


def look_for_seats(seats, x, y):
    seats_len_y = len(seats) - 1
    seats_len_x = len(seats[0]) - 1
    occupied_seats = 0
    for x_modif in range(-1, 2):
        for y_modif in range(-1, 2):
            if x_modif == 0 and y_modif == 0:
                continue
            # print(f"{x=} {y=}")
            cur_x = x
            cur_y = y
            while True:
                cur_x += x_modif
                cur_y += y_modif
                # print(f"{x_modif=} {y_modif=}")
                # print(f"{cur_x=} {cur_y=}")
                # input()

                if (cur_x < 0 or cur_x > seats_len_x) or (cur_y < 0 or cur_y > seats_len_y):
                    break
                if seats[cur_y][cur_x] == "#":
                    occupied_seats += 1
                    break
                elif seats[cur_y][cur_x] == "L":
                    break
    # input(occupied_seats)
    return occupied_seats


def load_input():
    with open("input") as f:
        return [list(row.strip()) for row in f.readlines()]


if __name__ == '__main__':
    main()

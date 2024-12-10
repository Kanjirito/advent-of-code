#!/usr/bin/env python3

SHIP_ORIENTATION = 1
WAYPOINT_X = 10
WAYPOINT_Y = 1


def main():
    part_one()
    part_two()


def part_one():
    horizontal_moves = 0
    vertical_moves = 0
    with open("input") as f:
        for line in f:
            order = line[0]
            num = int(line[1:].strip())
            if order == "N":
                vertical_moves += num
            elif order == "S":
                vertical_moves -= num
            elif order == "E":
                horizontal_moves += num
            elif order == "W":
                horizontal_moves -= num
            elif order == "F":
                if SHIP_ORIENTATION == 0:
                    vertical_moves += num
                elif SHIP_ORIENTATION == 2:
                    vertical_moves -= num
                elif SHIP_ORIENTATION == 1:
                    horizontal_moves += num
                elif SHIP_ORIENTATION == 3:
                    horizontal_moves -= num
            elif order == "L" or order == "R":
                steps = num // 90
                rotate_ship(steps, order)
    print(abs(horizontal_moves) + abs(vertical_moves))


def part_two():
    global WAYPOINT_X
    global WAYPOINT_Y
    horizontal_moves = 0
    vertical_moves = 0
    with open("input") as f:
        for line in f:
            order = line[0]
            num = int(line[1:].strip())
            if order == "N":
                WAYPOINT_Y += num
            elif order == "S":
                WAYPOINT_Y -= num
            elif order == "E":
                WAYPOINT_X += num
            elif order == "W":
                WAYPOINT_X -= num
            elif order == "F":
                horizontal_moves += num * WAYPOINT_X
                vertical_moves += num * WAYPOINT_Y
            elif order == "L" or order == "R":
                steps = num // 90
                rotate_waypoint(steps, order)
    print(abs(horizontal_moves) + abs(vertical_moves))


def rotate_ship(steps, direction):
    global SHIP_ORIENTATION
    if direction == "L":
        SHIP_ORIENTATION = (SHIP_ORIENTATION - steps) % 4
    elif direction == "R":
        SHIP_ORIENTATION = (SHIP_ORIENTATION + steps) % 4


def rotate_waypoint(steps, direction):
    global WAYPOINT_X
    global WAYPOINT_Y
    for x in range(steps):
        if direction == "L":
            WAYPOINT_X, WAYPOINT_Y = -WAYPOINT_Y, WAYPOINT_X
        elif direction == "R":
            WAYPOINT_X, WAYPOINT_Y = WAYPOINT_Y, -WAYPOINT_X


if __name__ == '__main__':
    main()

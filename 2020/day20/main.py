#!/usr/bin/env python3
import pprint

PP = pprint.PrettyPrinter(indent=4)
TILES = {}


class Tile:
    def __init__(self, data):
        self.id = int(data[5:9])
        self.tiles = [list(x) for x in data.split("\n")[1:]]

    @property
    def left(self):
        return [x[0] for x in self.tiles]

    @property
    def right(self):
        return [x[-1] for x in self.tiles]

    @property
    def top(self):
        return self.tiles[0]

    @property
    def bottom(self):
        return self.tiles[-1]

    def flip_vertical(self):
        self.tiles = list(reversed(self.tiles))

    def flip_horizontal(self):
        self.tiles = [list(reversed(row)) for row in self.tiles]

    def rotate_clockwise(self):
        self.tiles = list(zip(*reversed(self.tiles)))

    def rotate_counter_clockwise(self):
        self.tiles = list(zip(*self.tiles))[::-1]

    def __str__(self):
        string = ""
        for row in self.tiles:
            for char in row:
                string += char
            string += "\n"
        return string

    def __repr__(self):
        return self.__str__()


def main():
    load_input()
    # PP.pprint(TILES)
    # print(len(TILES))


def load_input():
    global TILES
    with open("input") as f:
        for tile in f.read().split("\n\n"):
            # print(tile)
            tile_cls = Tile(tile)
            TILES[tile_cls.id] = tile_cls


if __name__ == '__main__':
    main()
    a = TILES[1559]

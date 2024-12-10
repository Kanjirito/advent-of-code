#!/usr/bin/env python3

INPUT = [1, 2, 16, 19, 18, 0]
LAST_SPOKEN = {}


def main():
    print(solve(2020))
    print(solve(30000000))


def solve(number):
    spoken_dict = {}
    for n, num in enumerate(INPUT):
        spoken_dict[num] = n
    last_said = INPUT[-1]
    for n in range(len(INPUT), number):
        if last_said in spoken_dict:
            diff = n - 1 - spoken_dict[last_said]
            spoken_dict[last_said] = n - 1
            last_said = diff
        else:
            spoken_dict[last_said] = n - 1
            last_said = 0
    return last_said


if __name__ == '__main__':
    main()

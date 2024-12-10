#!/usr/bin/env python3


def main():
    numbers = load_input()
    # device_joltage = numbers[-1]
    print(numbers)
    current_joltage = 0
    diff_1 = 0
    diff_3 = 1
    for jolt in numbers:
        curr_diff = jolt - current_joltage
        if curr_diff == 1:
            diff_1 += 1
        elif curr_diff == 3:
            diff_3 += 1
        current_joltage = jolt
    print(diff_1 * diff_3)

    counter = {0: 1}

    for jolt in numbers:
        counter[jolt] = counter.get(jolt - 1, 0) + counter.get(jolt - 2, 0) + counter.get(jolt - 3, 0)
    print(counter[numbers[-1]])


def load_input():
    with open("input") as f:
        numbers = [int(num) for num in f.readlines()]
        numbers.sort()
        return numbers


if __name__ == '__main__':
    main()

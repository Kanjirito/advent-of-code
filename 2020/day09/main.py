#!/usr/bin/env python3


def main():
    numbers = load_rules()
    to_find = check(numbers)
    print(find_range(numbers, to_find))


def find_range(numbers, to_find):
    for low in range(len(numbers) - 2):
        for high in range(low + 1, len(numbers) - 1):
            if sum(numbers[low:high]) == to_find:
                return min(numbers[low:high]) + max(numbers[low:high])


def check(numbers):
    current_index = 25
    while current_index < len(numbers):
        checked_num = numbers[current_index]
        found = False
        for num_1 in numbers[current_index - 25: current_index]:
            for num_2 in numbers[current_index - 25: current_index]:
                if num_1 == num_2:
                    continue
                elif (num_2 + num_1) == checked_num:
                    found = True
                    current_index += 1
                    break
            if found:
                break
        else:
            return checked_num


def load_rules():
    with open("input") as f:
        return [int(num) for num in f.readlines()]


if __name__ == '__main__':
    main()

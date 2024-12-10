#!/usr/bin/env python3
import re

PARENTS_REG = re.compile(r"\([\d\+\*\s]*\)")
ADDITION_REG = re.compile(r"(\d+) [\+] (\d+)")
MULT_REG = re.compile(r"(\d+) [\*] (\d+)")


def main():
    inp = load_input()
    print(sum(part_one(inp)))
    print(sum(part_two(inp)))


def part_one(inp):
    results = []
    for line in inp:
        no_parenths = remove_parenths(line)
        results.append(int(calculate(no_parenths)))
    return results


def calculate(string):
    list_ = string.split(" ")
    while len(list_) != 1:
        num_1 = int(list_[0])
        num_2 = int(list_[2])
        if list_[1] == "+":
            list_ = [str(num_1 + num_2), *list_[3:]]
        elif list_[1] == "*":
            list_ = [str(num_1 * num_2), *list_[3:]]
    return "".join(list_)


def part_two(inp):
    results = []
    for line in inp:
        while (result := PARENTS_REG.search(line)):
            start = result.start()
            end = result.end()
            num = calculate_2(line[start + 1:end - 1])
            line = line[:start] + num + line[end:]
        results.append(int(calculate_2(line)))
    return results


def calculate_2(string):
    while (add_result := ADDITION_REG.search(string)):
        num_1 = int(add_result.group(1))
        num_2 = int(add_result.group(2))
        start = add_result.start()
        end = add_result.end()
        string = f"{string[:start]}{num_1 + num_2}{string[end:]}"
    while (add_result := MULT_REG.search(string)):
        num_1 = int(add_result.group(1))
        num_2 = int(add_result.group(2))
        start = add_result.start()
        end = add_result.end()
        string = f"{string[:start]}{num_1 * num_2}{string[end:]}"
    # while len(list_) != 1:
    #     num_1 = int(list_[0])
    #     num_2 = int(list_[2])
    #     if list_[1] == "+":
    #         list_ = [str(num_1 + num_2), *list_[3:]]
    #     elif list_[1] == "*":
    #         list_ = [str(num_1 * num_2), *list_[3:]]
    return string


def remove_parenths(line):
    while (result := PARENTS_REG.search(line)):
        start = result.start()
        end = result.end()
        num = calculate(line[start + 1:end - 1])
        line = line[:start] + num + line[end:]
    return line


def load_input():
    with open("input") as f:
        return [line.strip() for line in f.readlines()]


if __name__ == '__main__':
    main()

#!/usr/bin/env python3
import re

VALID_NUMBERS = set()
INPUT = []
MY_TICKET = []
VALID_TICKETS = []
RULES = {}


def main():
    load_input()
    # print(RULES)
    print(part_one())
    print(part_two())


def part_one():
    global INPUT, VALID_NUMBERS, VALID_TICKETS
    invalid = []
    for ticket in INPUT[25:]:
        # input(ticket)
        nums = [int(n) for n in ticket.strip().split(",")]
        for n in nums:
            # input(n)
            if n not in VALID_NUMBERS:
                invalid.append(n)
                break
        else:
            VALID_TICKETS.append(nums)
            VALID_TICKETS.append(MY_TICKET)
    return sum(invalid)


def part_two():
    global VALID_NUMBERS, VALID_TICKETS, RULES
    valid = {category: list(range(len(MY_TICKET))) for category in RULES}
    for n in range(len(MY_TICKET)):
        for category, valid_numbers in RULES.items():
            for row in VALID_TICKETS:
                if row[n] not in valid_numbers:
                    valid[category].remove(n)
                    break
    order = {}
    taken = []
    for x in sorted(valid, key=lambda key: len(valid[key])):
        for n in valid[x]:
            if n not in taken:
                order[n] = x
                taken.append(n)
    suma = 1
    for n, category in order.items():
        if category.startswith("departure"):
            suma *= MY_TICKET[n]
    return suma


def load_input():
    global VALID_NUMBERS, INPUT, MY_TICKET, RULES
    ranges_reg = re.compile(r"(.*): (\d*)-(\d*) or (\d*)-(\d*)")
    with open("input") as f:
        INPUT = f.readlines()
    MY_TICKET = [int(num) for num in INPUT[22].strip().split(",")]
    for line in INPUT[:20]:
        reg = ranges_reg.search(line)
        category = reg.group(1)
        RULES[category] = set()
        first = int(reg.group(2))
        second = int(reg.group(3))
        for n in range(first, second + 1):
            VALID_NUMBERS.add(n)
            RULES[category].add(n)
        third = int(reg.group(4))
        forth = int(reg.group(5))
        for n in range(third, forth + 1):
            VALID_NUMBERS.add(n)
            RULES[category].add(n)


if __name__ == '__main__':
    main()

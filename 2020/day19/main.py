#!/usr/bin/env python3
import re
import pprint
from itertools import product
from functools import cache
from copy import deepcopy

PP = pprint.PrettyPrinter(indent=4)
MESSAGES = []
RULES = {}
DIGIT_REG = re.compile(r"(\d+)")


def main():
    load_input()
    part_one()
    part_two()


def part_one():
    zero_rules = set(resolve_rules(0, 1))
    count = 0
    for message in MESSAGES:
        if message in zero_rules:
            count += 1
    print(count)


def part_two():
    rules_31 = resolve_rules(31, 2)
    pattern_len = len(rules_31[0])
    rules_31 = set(rules_31)
    rules_42 = set(resolve_rules(42, 2))

    counter = 0
    for message in MESSAGES:
        message_parts = [message[i:i + pattern_len] for i in range(0, len(message), pattern_len)]

        rule_31_counter = 0
        for part in message_parts[::-1]:
            if part in rules_31:
                rule_31_counter += 1
            else:
                break

        if 0 < rule_31_counter < len(message_parts) / 2 and all(part in rules_42 for part in message_parts[:-rule_31_counter]):
            counter += 1
    print(counter)


@cache
def resolve_rules(rule_id, rules_to_use):
    if rules_to_use == 1:
        rule_set = RULES
    else:
        rule_set = RULES_2
    if isinstance(rule_set[rule_id], str):
        return [rule_set[rule_id]]
    solutions = []
    inner_solutions = []
    for rule in rule_set[rule_id][0]:
        inner_solutions.append(resolve_rules(rule, rules_to_use))
    for solution in product(*inner_solutions):
        solutions.append("".join(solution))
    if rule_set[rule_id][1] is not None:
        inner_solutions = []
        for rule in rule_set[rule_id][1]:
            inner_solutions.append(resolve_rules(rule, rules_to_use))
        for solution in product(*inner_solutions):
            solutions.append("".join(solution))
    return solutions


def load_input():
    global MESSAGES, RULES, RULES_2
    with open("input") as f:
        lines = f.read().split("\n")
    MESSAGES = lines[137:]
    for rule in lines[:136]:
        split_rule = rule.split(": ")
        rule_num = int(split_rule[0])
        if split_rule[1].startswith("\""):
            rules = split_rule[1].replace("\"", "")
        elif "|" in split_rule[1]:
            rule_sets = split_rule[1].split("|")
            rules_1 = [int(num) for num in DIGIT_REG.findall(rule_sets[0])]
            rules_2 = [int(num) for num in DIGIT_REG.findall(rule_sets[1])]
            rules = [rules_1, rules_2]
        else:
            rules = [[int(num) for num in DIGIT_REG.findall(split_rule[1])], None]
        RULES[rule_num] = rules
    RULES_2 = deepcopy(RULES)
    RULES_2[11] = [[42, 31], [42, 11, 31]]


if __name__ == '__main__':
    main()

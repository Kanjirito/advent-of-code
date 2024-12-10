#!/usr/bin/env python3
import re
from json import dump

RULES = {}

with open("input") as f:
    for line in f:
        line = line.strip().split("bags contain")
        outer = line[0].strip()
        inner = line[1].strip()

        if inner == "no other bags.":
            RULES[outer] = []
            continue
        else:
            reg = re.compile(r"(\d) (.*?) bags?")
            for bag in reg.finditer(inner):
                current_rules = RULES.get(outer, [])
                current_rules.append((bag.group(2), int(bag.group(1))))
                RULES[outer] = current_rules

with open("rules", "w") as f:
    dump(RULES, f, indent=4)

POSSIBLE = set()
BAG_COUNT = 0


def main():
    print(find_bags("shiny gold"))
    pass


def find_bags(bag_name):
    count = 0
    bags_directly_inside = 0
    if not RULES[bag_name]:
        return bags_directly_inside

    for rule in RULES[bag_name]:
        bags_directly_inside += rule[1]
        count += rule[1] * find_bags(rule[0])
    return count + bags_directly_inside


if __name__ == '__main__':
    main()

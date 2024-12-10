#!/usr/bin/env python3


def main():
    rules, to_change = load_rules()
    modified_rules = rules.copy()
    result = execute(modified_rules)
    for index in to_change:
        modified_rules = rules.copy()
        if modified_rules[index][0] == "nop":
            modified_rules[index] = ("jmp", modified_rules[index][1])
        else:
            modified_rules[index] = ("nop", modified_rules[index][1])
        result = execute(modified_rules)
        if result is not False:
            print(result)
            break


def execute(rules):
    next_line_to_read = 0
    accumulator = 0
    already_executed = set()
    while next_line_to_read < len(rules) - 1:
        if next_line_to_read in already_executed:
            return False
        else:
            already_executed.add(next_line_to_read)

        current_rule = rules[next_line_to_read]
        if current_rule[0] == "acc":
            accumulator += current_rule[1]
            next_line_to_read += 1
            continue
        elif current_rule[0] == "nop":
            next_line_to_read += 1
            continue
        elif current_rule[0] == "jmp":
            next_line_to_read += current_rule[1]
            continue
    return accumulator


def load_rules():
    with open("input") as f:
        rules = []
        to_change = []
        for n, line in enumerate(f.readlines()):
            split = line.strip().split(" ")
            command = split[0]
            num = int(split[1])
            rules.append((command, num))
            if command == "jmp" or command == "nop":
                to_change.append(n)
        return rules, to_change[::-1]


if __name__ == '__main__':
    main()

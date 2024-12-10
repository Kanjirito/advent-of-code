#!/usr/bin/env python3


def main():
    right_count = 0
    with open("input") as f:
        for line in f:
            line_s = line.split(":")
            rule = parse_rule(line_s[0])
            password = line_s[1].strip()

            letter_count = 0
            for letter in password:
                if letter == rule["letter"]:
                    letter_count += 1
            if rule["min"] <= letter_count <= rule["max"]:
                right_count += 1
    print(right_count)


def parse_rule(rule):
    split = rule.split(" ")
    letter = split[1]
    nums = split[0].split("-")
    min_ = int(nums[0])
    max_ = int(nums[1])
    return {"letter": letter,
            "min": min_,
            "max": max_}


if __name__ == '__main__':
    main()

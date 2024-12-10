#!/usr/bin/env python3
from collections import Counter


def main():
    with open("input") as f:
        groups = f.read().split("\n\n")

    group_answers = []
    for group in groups:
        anwsers = Counter()
        people = group.splitlines()
        people_len = len(people)
        for person in people:
            anwsers.update(person)

        counter = 0
        for key, value in anwsers.items():
            if value == people_len:
                counter += 1
        group_answers.append(counter)
    print(sum(group_answers))


if __name__ == '__main__':
    main()

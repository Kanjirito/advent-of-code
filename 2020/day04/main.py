#!/usr/bin/env python3
import re

REQUIRED = {
    'byr': lambda x: len(x) <= 4 and 2002 >= int(x) >= 1920,
    'iyr': lambda x: len(x) <= 4 and 2020 >= int(x) >= 2010,
    'eyr': lambda x: len(x) <= 4 and 2030 >= int(x) >= 2020,
    'hgt': lambda x: (x.endswith('cm') and 193 >= int(x[:-2]) >= 150) or (x.endswith('in') and 76 >= int(x[:-2]) >= 59),
    'hcl': lambda x: re.match(r'^#[a-f\d]{6}$', x) != None,
    'ecl': lambda x: x in ['amb','blu','brn','gry','grn','hzl','oth'],
    'pid': lambda x: len(x) == 9 and x.isdigit(),
}


def main():
    passports = []
    with open("input") as f:
        passport = {}
        for line in f:
            if line == "\n":
                passports.append(passport)
                passport = {}
            else:
                pairs = line.strip().split(" ")
                for pair in pairs:
                    key, value = pair.split(":")
                    passport[key] = value
        passports.append(passport)

    valid_passports = 0
    validated_passwords = 0
    for passport in passports:
        for key in REQUIRED:
            if key not in passport:
                break
        else:
            valid_passports += 1
            if all(REQUIRED[key](passport[key]) for key in REQUIRED):
                validated_passwords += 1
    print(valid_passports, validated_passwords)


if __name__ == '__main__':
    main()

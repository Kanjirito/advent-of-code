#!/usr/bin/env python3

def main():
    with open("input") as f:
        lines = f.read().splitlines()
    for num in lines:
        for num_2 in lines:
            for num_3 in lines:
                if (int(num) + int(num_2) + int(num_3)) == 2020:
                    print(int(num) * int(num_2) * int(num_3))
                    break
            else:
                continue
            break
        else:
            continue
        break


if __name__ == '__main__':
    main()

#!/usr/bin/env python3
import re

MEMORY = {}
MASK = ""
with open("input") as f:
    INPUT = f.readlines()
MEM_REG = re.compile(r"mem\[(\d*)\] = (\d*)")
MASK_REG = re.compile(r"mask = ([1,X,0]*)")


def main():
    part_one()
    part_two()


def part_one():
    global MASK, MEMORY
    for line in INPUT:
        if line.startswith("mask"):
            MASK = MASK_REG.search(line).group(1)
            continue
        mem_reg = MEM_REG.search(line)
        mem_address = mem_reg.group(1)
        mem_value = int(mem_reg.group(2))
        mem_value_binary = f"{mem_value:036b}"
        new_binary = ""
        for digit, mask in zip(mem_value_binary, MASK):
            if mask != "X":
                new_binary += mask
            else:
                new_binary += digit
        new_num = int(new_binary, 2)
        MEMORY[mem_address] = new_num
    print(sum(MEMORY.values()))


def part_two():
    global MASK, MEMORY
    MEMORY = {}
    for line in INPUT:
        if line.startswith("mask"):
            MASK = MASK_REG.search(line).group(1)
            continue
        mem_reg = MEM_REG.search(line)
        mem_address = int(mem_reg.group(1))
        mem_addr_binary = f"{mem_address:036b}"
        mem_value = int(mem_reg.group(2))

        new_num = ""
        for digit, mask in zip(mem_addr_binary, MASK):
            if mask == "0":
                new_num += digit
            else:
                new_num += mask
        replace_X(new_num, mem_value)
    print(sum(MEMORY.values()))


def replace_X(addr, value):
    if "X" not in addr:
        MEMORY[addr] = value
    else:
        replace_X(addr.replace("X", "0", 1), value)
        replace_X(addr.replace("X", "1", 1), value)
    pass


if __name__ == '__main__':
    main()

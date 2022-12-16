import numpy
import math
import json
from collections import deque

with open('input.csv') as file:
    input = [line.strip() for line in file]

input_array = [input[i:i + 2] for i in range(0, len(input), 3)]


def compare_list(left: list, right: list):
    for i, left_item in enumerate(left):
        if len(right) <= i:
            return -1
        right_item = right[i]
        if type(left_item) is list:
            if type(right_item) is list:
                order = compare_list(left_item, right_item)
                if order != 0:
                    return order
            else:
                order = compare_list(left_item, [right_item])
                if order != 0:
                    return order
        elif type(right_item) is list:
            order = compare_list([left_item], right_item)
            if order != 0:
                return order
        elif left_item < right_item:
            return 1
        elif left_item > right_item:
            return -1
        else:
            continue
    if len(left) == len(right):
        return 0
    else:
        return 1


sum = 0
for index, input2 in enumerate(input_array, 1):
    both_lists = []
    for row in input2:
        list_all = []
        list_stack = [list_all]
        current_number = ''
        for c in row:
            if c.isnumeric():
                current_number += c
                continue
            else:
                if len(current_number) != 0:
                    list_stack[-1].append(int(current_number))
                current_number = ''
            if c == "[":
                new_list = []
                list_stack[-1].append(new_list)
                list_stack.append(new_list)
            elif c == "]":
                list_stack.pop()
            elif c == ",":
                continue

        both_lists.append(list_all[0])

    result = compare_list(both_lists[0], both_lists[1])

    sum += index if result == 1 else 0
    if (result == -1):
        print(result == 1, index, both_lists[0], ":", both_lists[1])
    continue


print(sum)

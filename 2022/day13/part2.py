import numpy
import math
import json
from collections import deque

input_array = numpy.genfromtxt('input.csv', delimiter='\n', dtype='unicode')


class PacketList:
    def __init__(self, packet_list):
        self.packet_list = packet_list

    def __lt__(self, other):
        return compare_list(self.packet_list, other.packet_list) == -1

    def __eq__(self, other):
        return compare_list(self.packet_list, other.packet_list) == 0

    def __str__(self):
        return str(self.packet_list)


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


all_packets_list = []

for row in input_array:
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

    all_packets_list.append(PacketList(list_all))

divider_packets = [PacketList([[2]]), PacketList([[6]])]

all_packets_list += divider_packets
all_packets_list.sort(reverse=True)

first_index = 0
second_index = 0
for i, packet in enumerate(all_packets_list, 1):
    if packet == divider_packets[0]:
        first_index = i
    elif packet == divider_packets[1]:
        second_index = i

print(first_index * second_index)

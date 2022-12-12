import numpy
import math

with open('input.csv') as file:
    input = [line.strip() for line in file]

input_array = [input[i:i + 7] for i in range(0, len(input), 7)]


class Monkey:

    def __init__(self):
        self.items = []
        self.divisor = 0
        self.monkey_index = {}
        self.inspect_count = 0
        self.operation_str = ''


def operation(row: str, item: int):
    split_row = row.split()

    if split_row[4] == '*':
        if split_row[5] == 'old':
            new_item = item * item
        else:
            new_item = item * int(split_row[5])
    else:
        new_item = item + int(split_row[5])
    return int(new_item / 3)


monkey_list = [Monkey() for _ in range(len(input_array))]
for i, rows in enumerate(input_array):
    for item in rows[1].split()[2:]:
        monkey_list[i].items.append(int(item.strip(',')))
    monkey_list[i].divisor = int(rows[3].split()[3])
    monkey_list[i].monkey_index = {
        True: int(rows[4].split()[5]), False: int(rows[5].split()[5])}
    monkey_list[i].operation_str = rows[2]

for round in range(20):
    for monkey in monkey_list:
        for item in monkey.items:
            new_item = operation(monkey.operation_str, item=item)
            condition = new_item % monkey.divisor == 0
            monkey_list[monkey.monkey_index[condition]].items.append(new_item)
            monkey.inspect_count += 1
        monkey.items.clear()

max_inspects = [x.inspect_count for x in monkey_list]
max_inspects.sort(reverse=True)

print(max_inspects[0] * max_inspects[1])

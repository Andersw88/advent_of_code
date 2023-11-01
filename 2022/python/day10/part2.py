import numpy
import math

input = numpy.genfromtxt('input.csv', delimiter=',', dtype='unicode')

cycles = [1]
for row in input:
    split_row = row.split()
    if split_row[0] == 'noop':
        cycles.append(cycles[-1])
    elif split_row[0] == 'addx':
        cycles.append(cycles[-1])
        cycles.append(cycles[-1] + int(split_row[1]))

draw_cycle = 0
for row in range(0, 220, 40):
    for column in range(0, 40):
        if column <= cycles[draw_cycle] + 1 and column >= cycles[draw_cycle] - 1:
            print("#", end="")
        else:
            print(".", end="")
        draw_cycle += 1
    print()

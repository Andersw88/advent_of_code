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
cycles.insert(0, 0)

scan_cycles = [cycles[20], cycles[60], cycles[100],
               cycles[140], cycles[180], cycles[220]],

signal_strengths = [cycles[20] * 20, cycles[60] * 60, cycles[100] * 100,
                    cycles[140] * 140, cycles[180] * 180, cycles[220] * 220]
print(scan_cycles)
print(signal_strengths)
total = sum(signal_strengths)
print(total)

import numpy
import math

input = numpy.genfromtxt('input.csv', delimiter=',', dtype='unicode')

moves = []

dir_map = {
    'R': (1, 0,),
    'L': (-1, 0,),
    'U': (0, 1,),
    'D': (0, -1,),
}

moves = [dir_map[d]
         for row in input
         for d in (row.split()[0],) * int(row.split()[1])]

tail_positions = set()
tail_list = numpy.zeros((10, 2,), dtype=int)

for move in moves:
    tail_list[0] += move
    for index in range(1, len(tail_list)):
        diff = tail_list[index - 1] - tail_list[index]

        if abs(diff[0]) > 1 or abs(diff[1]) > 1:
            tail_list[index] += [int(numpy.sign(diff[0])),
                                 int(numpy.sign(diff[1]))]

    tail_positions.add(tuple(tail_list[9]))

print(len(tail_positions))

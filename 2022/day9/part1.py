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

head = numpy.zeros(2, dtype=int)
tail = head.copy()
tail_positions.add(tuple(tail))

for move in moves:
    head += move
    diff = head - tail

    if abs(diff[0]) > 1 or abs(diff[1]) > 1:
        tail += [int(numpy.sign(diff[0])),
                 int(numpy.sign(diff[1]))]

    tail_positions.add(tuple(tail))

print(len(tail_positions))

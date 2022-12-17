import numpy
import math
import json
import sys
from collections import deque

input = numpy.genfromtxt('input.csv', delimiter='\n', dtype='unicode')
numpy.set_printoptions(threshold=numpy.inf, linewidth=800, suppress=True)

rock_map = numpy.zeros((400, 200), dtype=int)
floor = 0

for row in input:
    split_row = row.split()

    lines = numpy.asarray([[int(x) for x in xy.split(',')]
                           for xy in split_row[::2]])

    for line_start, line_end in [(lines[i], lines[i + 1]) for i in range(len(lines) - 1)]:

        range_x = [line_start[0] - 300, line_end[0] - 300]
        range_x.sort()

        range_y = [line_start[1], line_end[1]]
        range_y.sort()

        if floor < range_y[1] + 2:
            floor = range_y[1] + 2
        for x in range(range_x[0], range_x[1] + 1):
            for y in range(range_y[0], range_y[1] + 1):
                rock_map[x, y] = 1

rc2 = rock_map[0:1, 0:1]

fall_directions = numpy.asarray([[0, 1], [-1, 1], [1, 1]])

sand_particle_start = numpy.asarray([200, 0])

sand_fell_off = False
sand_particle = sand_particle_start.copy()
num_particles = 0
while not sand_fell_off:
    moved = False
    for dir in fall_directions:
        new_pos = sand_particle + dir

        if new_pos[1] >= floor:
            continue

        if rock_map[tuple(new_pos)] == 0:
            sand_particle = new_pos
            moved = True
            break
    if not moved:
        if (sand_particle == sand_particle_start).all():
            sand_fell_off = True

        rock_map[tuple(sand_particle)] = 2
        sand_particle = sand_particle_start.copy()
        num_particles += 1


print(numpy.transpose(rock_map[120:250, 0:40]))
print(numpy.transpose(rock_map[120:250, 40:100]))
print(numpy.transpose(rock_map[120:250, 100:140]))
print(numpy.transpose(rock_map[120:250, 140:floor + 1]))
print(floor)
print(num_particles)

import numpy
import math
from collections import deque

input = numpy.genfromtxt('input.csv', delimiter=',', dtype='unicode')

terrain_map = [ord(c) - ord('a') for row in input for c in row]
width = len(input[0])
height = len(input)

start = terrain_map.index(ord('S') - ord('a'))
end = terrain_map.index(ord('E') - ord('a'))

terrain_map[start] = 0
terrain_map[end] = ord('z') - ord('a')

terrain_map = numpy.asarray(terrain_map, dtype=int)
terrain_map = terrain_map.reshape(height, width)
explored_map = numpy.zeros(terrain_map.shape, dtype=bool)

start = numpy.asarray((start / width, start % width), dtype=int)
end = numpy.asarray((end / width, end % width), dtype=int)
print(start, end)
print(width, height)
print(terrain_map)

node_deque = deque()
node_deque.append((start, 0,))

directions = [(1, 0), (-1, 0), (0, 1), (0, -1)]

while True:
    next_node = node_deque.pop()
    old_height = terrain_map[tuple(next_node[0])]

    if (next_node[0] == end).all():
        break

    for dir in directions:
        new_pos = next_node[0] + dir
        if new_pos[0] >= 0 and new_pos[0] < height and new_pos[1] >= 0 and new_pos[1] < width:
            new_height = terrain_map[tuple(new_pos)]
            explored = explored_map[tuple(new_pos)]
            if new_height <= old_height + 1 and not explored:
                node_deque.appendleft((new_pos, next_node[1] + 1))
                explored_map[tuple(new_pos)] = True

print(next_node[1])

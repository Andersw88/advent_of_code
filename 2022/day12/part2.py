import numpy
import math
from collections import deque


input = numpy.genfromtxt('input.csv', delimiter=',', dtype='unicode')

terrain_map_linear = [ord(c) - ord('a') for row in input for c in row]
width = len(input[0])
height = len(input)

start_S = terrain_map_linear.index(ord('S') - ord('a'))
terrain_map_linear[start_S] = 0
starts = [i for i, start in enumerate(terrain_map_linear) if start == 0]
end = terrain_map_linear.index(ord('E') - ord('a'))
terrain_map_linear[end] = ord('z') - ord('a')
end = numpy.asarray((end / width, end % width), dtype=int)

distances = []

print(len(starts))
number_of_starts = 0

for start in starts:
    number_of_starts += 1

    terrain_map = numpy.asarray(terrain_map_linear, dtype=int)
    terrain_map = terrain_map.reshape(height, width)
    explored_map = numpy.zeros(terrain_map.shape, dtype=bool)
    start = numpy.asarray((start / width, start % width), dtype=int)

    node_deque = deque()
    node_deque.append((start, 0,))

    directions = [(1, 0), (-1, 0), (0, 1), (0, -1)]

    while True:
        if len(node_deque) == 0:
            break
        next_node = node_deque.pop()
        old_height = terrain_map[tuple(next_node[0])]

        if (next_node[0] == end).all():
            distances.append(next_node[1])
            break

        for dir in directions:
            new_pos = next_node[0] + dir
            if new_pos[0] >= 0 and new_pos[0] < height and new_pos[1] >= 0 and new_pos[1] < width:
                new_height = terrain_map[tuple(new_pos)]
                explored = explored_map[tuple(new_pos)]
                if new_height <= old_height + 1 and not explored:

                    if new_height == 0:
                        flat_index = new_pos[0] * width + new_pos[1]
                        if flat_index in starts:
                            starts.remove(flat_index)
                        node_deque.appendleft((new_pos, 0))
                    else:
                        node_deque.appendleft((new_pos, next_node[1] + 1))

                    explored_map[tuple(new_pos)] = True


print(number_of_starts)
distances.sort()
print(distances[0])

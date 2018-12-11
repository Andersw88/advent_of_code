import numpy as np
import parse
from collections import deque
from timeit import default_timer as timer

input = np.genfromtxt('input.csv', delimiter='\n',
                      dtype='unicode', comments=None)

parse_string = '{:d}, {:d}'

input_array = [parse.parse(parse_string, x).fixed for x in input]

max_x = max(input_array, key=lambda x: x[0])[0] + 1
max_y = max(input_array, key=lambda x: x[1])[1] + 1

grid = np.zeros([max_x, max_y], dtype=int)

for x in range(max_x):
    for y in range(max_y):
        for point in input_array:
            grid[x, y] += abs(point[0] - x) + abs(point[1] - y)

count = (grid < 10000).sum()

print(count)

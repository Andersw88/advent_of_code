import numpy as np
import parse
from collections import deque
from timeit import default_timer as timer


def proccess_point(grid, points, point, curr_index, curr_dist):
    grid_element = grid[point[0], point[1]]

    if grid_element[0] == curr_index:
        return
    if grid_element[1] < curr_dist and grid_element[0] != 0:
        return
    if grid_element[1] == curr_dist:
        grid_element[0] = -1
    # elif grid_element[1] > curr_dist:
    #     print('error')
    elif grid_element[0] == 0:
        points.appendleft((point, curr_index, curr_dist))
        grid[point[0], point[1]] = [curr_index, curr_dist]


input = np.genfromtxt('input.csv', delimiter='\n',
                      dtype='unicode', comments=None)

parse_string = '{:d}, {:d}'


input_array = [parse.parse(parse_string, x).fixed for x in input]

max_x = max(input_array, key=lambda x: x[0])[0] + 1
max_y = max(input_array, key=lambda x: x[1])[1] + 1

grid = np.zeros([max_x + 1, max_y + 1, 2], dtype=int)


for i, point in enumerate(input_array):
    grid[point[0]][point[1]] = [i + 1, 0]

points = deque(zip(input_array, range(
    1, len(input_array) + 1), [0] * len(input_array)))

while(points):
    curr_element = points.pop()
    point = curr_element[0]
    curr_index = curr_element[1]
    curr_dist = curr_element[2] + 1

    if point[0] + 1 <= max_x:
        proccess_point(
            grid, points, (point[0] + 1, point[1]), curr_index, curr_dist)

    if point[1] + 1 <= max_y:
        proccess_point(
            grid, points, (point[0], point[1] + 1), curr_index, curr_dist)

    if point[0] - 1 >= 0:
        proccess_point(
            grid, points, (point[0] - 1, point[1]), curr_index, curr_dist)

    if point[1] - 1 >= 0:
        proccess_point(
            grid, points, (point[0], point[1] - 1), curr_index, curr_dist)

    # for x in grid:
    #     print(*x, sep=" ")
    # print('')

counts = [0] * len(input_array)
infinite_points = [0] * (len(input_array) + 2)

for x in grid[0, :]:
    infinite_points[x[0]] += 1
for x in grid[max_x, :]:
    infinite_points[x[0]] += 1
for x in grid[:, 0]:
    infinite_points[x[0]] += 1
for x in grid[:, max_y]:
    infinite_points[x[0]] += 1


for id in grid.reshape(-1, 2):
    if(id[0] > 0) and infinite_points[id[0]] == 0:
        counts[id[0] - 1] += 1

print(max(counts))

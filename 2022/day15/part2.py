import numpy
import math
import json
import sys
import parse

input = numpy.genfromtxt('input.csv', delimiter='\n', dtype='unicode')
# numpy.set_printoptions(threshold=sys.maxsize, linewidth=800)

sensors = []

for row in input:
    sensor_x, sensor_y, beacon_x, beacon_y = parse.parse(
        "Sensor at x={:d}, y={:d}: closest beacon is at x={:d}, y={:d}", row)

    sensors.append([sensor_x, sensor_y, abs(
        sensor_x - beacon_x) + abs(sensor_y - beacon_y)])

sensors = numpy.asarray(sensors, dtype=int)

print(sensors)

max_index = 4000000
not_found_realy = False
y = 0
while y <= max_index and not not_found_realy:
    x = 0
    while x <= max_index and not not_found_realy:
        found = False
        for i, sensor in enumerate(sensors):
            skip = sensor[2] - (abs(sensor[0] - x) + abs(sensor[1] - y)) + 1
            if skip > 0:
                found = True
                x += skip
                break
        if not found:
            not_found_realy = True
            print(x, y)
            print(int(x * 4000000 + y))
            break
        if x >= max_index and y % 1000 == 0:
            print(x, y)
    y += 1

print(2949122 * 4000000 + 3041245)

import numpy
import math
import json
import sys
import parse

input = numpy.genfromtxt('input.csv', delimiter='\n', dtype='unicode')
numpy.set_printoptions(threshold=sys.maxsize, linewidth=800)

bounds = numpy.asarray([sys.maxsize, -sys.maxsize])


sensors = []
max_sensor_distances = []
# scan_level = 9
scan_level = 2000000

beacons_on_scan_level = set()

for row in input:
    sensor_x, sensor_y, beacon_x, beacon_y = parse.parse(
        "Sensor at x={:d}, y={:d}: closest beacon is at x={:d}, y={:d}", row)

    if beacon_y == scan_level:
        beacons_on_scan_level.add(beacon_x)
    if sensor_y == scan_level:
        beacons_on_scan_level.add(sensor_y)

    sensors.append([sensor_x, sensor_y])
    max_sensor_distances.append(
        abs(sensor_x - beacon_x) + abs(sensor_y - beacon_y))


sensors = numpy.asarray(sensors)

sensors2 = numpy.asarray([[sensor[0], max_sensor_distances[i] -
                           abs(sensor[1] - scan_level)] for i, sensor in enumerate(sensors)])

for sensor in sensors2:
    min_x = sensor[0] - sensor[1]
    max_x = sensor[0] + sensor[1]
    if min_x < bounds[0]:
        bounds[0] = min_x
    if max_x > bounds[1]:
        bounds[1] = max_x

num_sections = 0
for x in range(bounds[0] - 10, bounds[1] + 10):

    if x in beacons_on_scan_level:
        continue
    for i, sensor in enumerate(sensors2):
        if abs(sensor[0] - x) <= sensor[1]:
            num_sections += 1
            break

print(num_sections)
print(bounds, bounds[1] - bounds[0])

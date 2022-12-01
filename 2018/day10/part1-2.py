import numpy as np
import parse
from collections import deque
# from timeit import default_timer as timer
import time
import math
import matplotlib.pyplot as plt



input = np.genfromtxt('input.csv', delimiter='\n',
                      dtype='unicode', comments=None)

# position=< 9,  1> velocity=< 0,  2>
parse_string = 'position=<{:d}, {:d}> velocity=<{:d}, {:d}>'
compiled_string = parse.compile(parse_string)

input_array = [compiled_string.parse(x).fixed for x in input]

stars = [(x, y, dx, dy) for x,y,dx,dy in input_array]

star_closeness = 1
previous_star_closeness = 0
rate = 0.1
t = 0
times_same = 0


for x in range(1000):
    dt = int(star_closeness * rate)
    stars = [(x + dx * dt, y + dy * dt, dx, dy) for x,y,dx,dy in stars]

    star_closeness = sum([min([abs(s1[0] - s2[0]) + abs(s1[1] - s2[1]) for s1 in stars if s1 != s2]) for s2 in stars])

    t += dt
    print(star_closeness, t, dt)

    if star_closeness - previous_star_closeness > 0:
        rate = -rate/1.5

    if previous_star_closeness == star_closeness:
        times_same += 1
    else:
        times_same = 0

    previous_star_closeness = star_closeness

    if times_same == 5:
        break

final_stars_x = [x for x,y,dx,dy in stars]
final_stars_y = [-y for x,y,dx,dy in stars]


print("time:",t)
plt.scatter(final_stars_x,final_stars_y)
plt.show()


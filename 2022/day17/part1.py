import numpy as np
import math
import json
import sys
import parse

np.set_printoptions(threshold=sys.maxsize, linewidth=800)
with open('input.csv') as file:
    input = [line for line in file]


directions = np.array([c == '>' for c in input[0]], dtype=np.uint8)

rocks = [list(np.array([0b00111100], dtype=np.uint8)),

         list(np.array([0b00010000,
                        0b00111000,
                        0b00010000], dtype=np.uint8)),

         list(np.array([0b00001000,
                        0b00001000,
                        0b00111000], dtype=np.uint8)),

         list(np.array([0b00100000,
                        0b00100000,
                        0b00100000,
                        0b00100000], dtype=np.uint8)),

         list(np.array([0b00110000,
                        0b00110000], dtype=np.uint8)),
         ]


zero_byte = np.uint8(0b00000000)
full_byte = np.uint8(0b11111111)

rock_tower = [full_byte]

walls = [np.uint8(0b10000000), np.uint8(0b00000010)]


def collide_rock(rock: list[np.uint8], rock_slice: list[np.uint8]):
    collided = False
    for rock_row, tower_row in zip(rock, rock_slice):
        collided |= bool(rock_row & tower_row)
    return collided


current_rock_index = 0
pos_from_top = 0
rock = rocks[0].copy()
rock_tower.extend([zero_byte] * (3 + len(rock)))
highest_point = 0
rocks_tower_height = len(rock_tower) - 1

rocks_index = 1

iterations = 0

while True:

    start_top = -pos_from_top
    stop_top = -(pos_from_top + len(rock))
    rock_tower_slice = rock_tower[stop_top:start_top][::-1]
    rock_tower_slice_c = rock_tower[stop_top - 1:start_top - 1][::-1]

    # for j, row in enumerate(rock_tower[::-1]):
    #     if -j > stop_top and -j <= start_top:
    #         rock_index = j + stop_top
    #         print(np.unpackbits(row | rock[rock_index]))
    #     else:
    #         print(np.unpackbits(row))
    # print()

    if directions[iterations % len(directions)]:
        if not collide_rock(rock, [walls[1]] * len(rock)) and not collide_rock([np.uint8(x >> 1) for x in rock], rock_tower_slice):
            rock = [np.uint8(x >> 1) for x in rock]
    else:
        if not collide_rock(rock, [walls[0]] * len(rock)) and not collide_rock([np.uint8(x << 1) for x in rock], rock_tower_slice):
            rock = [np.uint8(x << 1) for x in rock]

    # for j, row in enumerate(rock_tower[::-1]):
    #     if -j > stop_top and -j <= start_top:
    #         rock_index = j + stop_top
    #         print(np.unpackbits(row | rock[rock_index]))
    #     else:
    #         print(np.unpackbits(row))
    # print()

    if collide_rock(rock, rock_tower_slice_c):

        rock_tower[stop_top:start_top] = [rock |
                                          stone for rock, stone in zip(rock[::-1], rock_tower_slice[::-1])]

        previous_rock_height = len(rock)
        rocks_tower_height = (highest_point + 3 + previous_rock_height)

        highest_point2 = max(
            highest_point, rocks_tower_height - pos_from_top)

        rock = rocks[rocks_index % len(rocks)].copy()

        rocks_tower_height_new = highest_point2 + 3 + len(rock)

        if rocks_tower_height_new > rocks_tower_height:
            rock_tower.extend(
                [zero_byte] * (rocks_tower_height_new - rocks_tower_height))
        elif rocks_tower_height > rocks_tower_height_new:
            del rock_tower[rocks_tower_height_new - rocks_tower_height:]
        highest_point = highest_point2

        pos_from_top = 0
        rocks_index += 1

        if rocks_index > 2022:
            break
    else:
        pos_from_top += 1

    iterations += 1


print(highest_point)

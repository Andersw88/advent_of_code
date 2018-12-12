import numpy as np
import parse

from itertools import chain

input = np.genfromtxt('input.csv', delimiter='\n',
                      dtype='unicode', comments=None)

# initial state: ##.#....#..#......#..######..#.####.....#......##.##.##...#..#....#.#.##..##.##.#.#..#.#....#.#..#.#
first_parse_string = 'initial state: {}'
# ##### => #
second_parse_string = '{} => {}'


initial_state_str = parse.parse(first_parse_string, input[0])[0]
num_generations_array = [20, 50000000000]
for num_generations in num_generations_array:
    state_array_length = 118

    state_array = np.fromiter(chain(
        (False for x in range(2)),
        (x == '#' for x in initial_state_str), 
        (False for y in range(state_array_length+1))
    ), dtype=bool)

    patten_array = np.zeros([pow(2,5)], dtype=np.uint8)

    for pattern, result in [parse.parse(second_parse_string, x) for x in input[1:]]:
        patten_array[np.packbits([0,0,0] + [c == '#' for c in pattern])] = result == '#'

    state_array_copy = np.array(state_array)

    # print(*['#' if x else '.' for x in state_array],sep="")

    stored_generation = 0
    for g in range(num_generations):

        state_array_copy = np.array(state_array)
        state_array[2:-2] = [patten_array[np.packbits([False,False,False, *x1[0]])] for x1 in zip(state_array[x:x+5] for x in range(len(state_array)-4))]

        stored_generation = g
        if (state_array[1:] == state_array_copy[:-1]).all():
            break

        # print(*['#' if x else '.' for x in state_array],sep="")

    # print(*['#' if x else '.' for x in state_array],sep="")

    pot_sum = sum([x - 3 + (num_generations - stored_generation) for x in range(len(state_array)) if state_array[x]])

    print(pot_sum)

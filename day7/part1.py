import numpy as np
import parse

input = np.genfromtxt('input.csv', delimiter='\n',
                      dtype='unicode', comments=None)

np.set_printoptions(threshold=np.nan, linewidth=1002)

# Step C must be finished before step A can begin.
parse_string = 'Step {} must be finished before step {} can begin.'

input_array = [parse.parse(parse_string, x).fixed for x in input]
input_array_ord = [(ord(x[0]) - 65, ord(x[1]) - 65) for x in input_array]

existing_letters = set()
num_letters = ord('Z') - ord('A')
dependency_arrays = np.zeros([num_letters, num_letters], dtype=bool)

for step, dep in input_array_ord:
    dependency_arrays[dep, step] = True
    existing_letters.add(step)
    existing_letters.add(dep)

procssed_steps = ""

step_set = set()


for i in range(len(existing_letters)):
    for step in range(num_letters):
        if step not in step_set and not (dependency_arrays[step]).any():
            for dep_array in dependency_arrays:
                dep_array[step] = False
            step_set.add(step)
            procssed_steps += (chr(step + 65))
            break

print(procssed_steps, len(procssed_steps))

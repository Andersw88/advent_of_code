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
num_letters = ord('Z') - ord('A') + 1
dependency_arrays = np.zeros([num_letters, num_letters], dtype=bool)

for step, dep in input_array_ord:
    dependency_arrays[dep, step] = True
    existing_letters.add(step)
    existing_letters.add(dep)

procssed_steps = ""
num_existing_letters = len(existing_letters)

step_set = set()

time_when_completed = np.full([num_existing_letters], -1)

workers = 5
time = 0
while len(procssed_steps) != num_existing_letters:
    for step in range(num_existing_letters):
        if time_when_completed[step] != -1 and time_when_completed[step] <= time:
            workers += 1
            time_when_completed[step] = -1
            for dep_array in dependency_arrays:
                dep_array[step] = False
            procssed_steps += (chr(step + 65))

    for step in range(num_existing_letters):
        if step not in step_set and not (dependency_arrays[step]).any() and workers > 0:
            time_when_completed[step] = time + step + 61
            workers -= 1
            step_set.add(step)
            # break
    
    
    time += 1

time -= 1

print(procssed_steps, len(procssed_steps), time)

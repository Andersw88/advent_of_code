import numpy as np
input = np.genfromtxt('input.csv',delimiter=',', dtype='unicode')

max_calories = 0
current_sum = 0

all_caliories = []
for input_string in input:
    if input_string == '':
        all_caliories.append(current_sum)
        current_sum = 0
    else:
        current_sum += int(input_string)
all_caliories.append(current_sum)
all_caliories.sort(reverse=True)

max_calories = sum(all_caliories[0:3])

print(max_calories)
import numpy as np
input = np.genfromtxt('input.csv',delimiter=',', dtype='unicode')

max_calories = 0
current_sum = 0
for input_string in input:
    if input_string == '':
        max_calories = max_calories if max_calories > current_sum else current_sum
        current_sum = 0
    else:
        current_sum += int(input_string)
max_calories = max_calories if max_calories > current_sum else current_sum

print(max_calories)
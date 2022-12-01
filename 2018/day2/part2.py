import numpy as np
input = np.genfromtxt('input.csv',delimiter='\n', dtype='unicode')

string_set = set()
result = None
for input_string in input:
    new_sting_set = set(string_set)
    for i in range(len(input_string)):
        new_string = input_string[:i] + input_string[(i+1):]
        if new_string in string_set:
            result = new_string
        else:
            new_sting_set.add(new_string)
    string_set = new_sting_set

print(result)

import numpy as np
input = np.genfromtxt('input.csv',delimiter='\n', dtype='unicode')

char2 = 0
char3 = 0

for input_string in input:
    char_dict = {}
    for c in input_string:
        if c in char_dict:
            char_dict[c] += 1
        else:
            char_dict[c] = 1

    char2_found = False
    char3_found = False
    for count in char_dict.values():
        if count == 2 and not char2_found:
            char2 += 1
            char2_found = True
        elif count == 3 and not char3_found:
            char3 += 1
            char3_found = True

print(char2 * char3)

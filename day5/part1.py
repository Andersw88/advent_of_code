import numpy as np
import parse
from timeit import default_timer as timer

def shrink_str(ord_list):
    curr_length = len(ord_list)
    prev_len = 0
    keep_list = np.ones(len(ord_list))

    while prev_len != curr_length:
        for i in range(curr_length - 1):
            if keep_list[i] and ((ord_list[i] == ord_list[i + 1] - 32) or (ord_list[i] == ord_list[i + 1] + 32)):
                keep_list[i:i+2] = 0

        ord_list = [e for i, e in enumerate(ord_list) if keep_list[i]]
        prev_len = curr_length
        curr_length = len(ord_list)
        keep_list[:] = 1
    return ord_list

input = None
with open('input.csv') as inputFile:
    input = list(inputFile.readlines()[0])

char_list = list(range(ord('A'), ord('Z') + 1))
results_list = [0] * len(char_list)

start = timer()

for index, c in enumerate(char_list):
    ord_list = [ord(x) for x in input if x != chr(c) and x != chr(c + 32)]

    ord_list = shrink_str(ord_list)
    
    results_list[index] = len(ord_list)

end = timer()

result = min(results_list)

print(result, end - start, results_list)
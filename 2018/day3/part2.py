import numpy as np
import parse
input = np.genfromtxt('input.csv',delimiter='\n', dtype='unicode', comments=None)

np.set_printoptions(threshold=np.nan, linewidth=1002)
#123 @ 3,2: 5x4
sheet_array = np.zeros([1000,1000], dtype=int)

candidates = np.ones(len(input) + 1, dtype=int)
candidates[0] = 0

parse_string = '#{} @ {},{}: {}x{}'
for i, line in enumerate(input):
    i2 = i + 1
    result = parse.parse(parse_string, line)
    x1 = int(result[1])
    y1 = int(result[2])
    x2 = int(result[3]) + x1
    y2 = int(result[4]) + y1
    conflict_array = sheet_array[x1:x2,y1:y2][sheet_array[x1:x2,y1:y2] > 0]

    if conflict_array.any():
        candidates[i2] = 0
        for conflict in conflict_array:
            candidates[conflict] = 0
    sheet_array[x1:x2,y1:y2] = i2

result = np.flatnonzero(candidates)
print(candidates)
print(result[0], len(result))

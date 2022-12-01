import numpy as np
import parse
input = np.genfromtxt('input.csv',delimiter='\n', dtype='unicode', comments=None)

np.set_printoptions(threshold=np.nan, linewidth=1002)
#123 @ 3,2: 5x4
sheet_array = np.zeros([1000,1000], dtype=int)

parse_string = '#{} @ {},{}: {}x{}'
for line in input:
    result = parse.parse(parse_string, line)
    x1 = int(result[1])
    y1 = int(result[2])
    x2 = int(result[3]) + x1
    y2 = int(result[4]) + y1
    sheet_array[x1:x2,y1:y2] += 1

count = (sheet_array > 1).sum()
print(count)

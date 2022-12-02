import numpy as np
input = np.genfromtxt('input.csv',delimiter='\n', dtype='unicode')

score = {
    'A X': 1 + 3,
    'A Y': 2 + 6,
    'A Z': 3,

    'B X': 1,
    'B Y': 2 + 3,
    'B Z': 3 + 6,

    'C X': 1 + 6,
    'C Y': 2,
    'C Z': 3 + 3},

total_score = 0
for row in input:
    print(score[row])
    total_score += score[row]

print(total_score)
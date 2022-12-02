import numpy as np
input = np.genfromtxt('input.csv',delimiter='\n', dtype='unicode')

score = {
    'A X': 3,
    'A Y': 1 + 3,
    'A Z': 2 + 6,

    'B X': 1,
    'B Y': 2 + 3,
    'B Z': 3 + 6,

    'C X': 2,
    'C Y': 3 + 3,
    'C Z': 1 + 6}

total_score = 0
for row in input:
    print(score[row])
    total_score += score[row]

print(total_score)
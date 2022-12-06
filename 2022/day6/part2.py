from collections import deque

with open('input.csv') as file:
    input = [line for line in file]

most_recent_chars = deque()

for index, char in enumerate(input[0]):
    most_recent_chars.append(char)
    if len(most_recent_chars) == 15:
        most_recent_chars.popleft()
        if len(set(most_recent_chars)) == 14:
            print(index + 1)
            break

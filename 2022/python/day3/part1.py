import numpy as np
input = np.genfromtxt('input.csv', delimiter='\n', dtype='unicode')

itemCountSet = set()


def charToPriority(char: str):
    if ord(char) > ord('Z'):
        return ord(char) - ord('a') + 1

    return ord(char) - ord('A') + 27


total_prio = 0
for row in input:
    half = int(len(row) / 2)
    for char in row[0:half]:
        prio = charToPriority(char)
        itemCountSet.add(prio)

    for char in row[half:len(row)]:
        prio = charToPriority(char)
        if prio in itemCountSet:
            print(prio, char)
            total_prio += prio
            itemCountSet.clear()
            break
    itemCountSet.clear()

print(total_prio)

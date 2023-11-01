import numpy as np
input = np.genfromtxt('input.csv', delimiter='\n', dtype='unicode')

itemCountSet = set()
itemCountSet2 = set()


def charToPriority(char: str):
    if ord(char) > ord('Z'):
        return ord(char) - ord('a') + 1

    return ord(char) - ord('A') + 27


total_prio = 0
for row in zip(input[0:len(input):3], input[1:len(input):3], input[2:len(input):3]):
    print(row[0:3])
    for char in row[0]:
        itemCountSet.add(charToPriority(char))

    for char in row[1]:
        itemCountSet2.add(charToPriority(char))

    for char in row[2]:
        prio = charToPriority(char)
        if prio in itemCountSet and prio in itemCountSet2:
            print(prio, char)
            total_prio += prio
            itemCountSet.clear()
            itemCountSet2.clear()
            break
    itemCountSet.clear()
    itemCountSet2.clear()

print(total_prio)

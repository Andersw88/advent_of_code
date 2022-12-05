import numpy as np
input = np.genfromtxt('input.csv', delimiter='\n', dtype='unicode')

itemCountSet = set()


def fillSet(rangeString):
    start, end = (int(x) for x in rangeString.split('-'))

    section = set()
    section.update(range(start, end + 1))
    return section


count = 0
for row in input:
    first, second = row.split(',')
    firstSection = fillSet(first)
    secondSection = fillSet(second)
    if (not firstSection.isdisjoint(secondSection)):
        count += 1

print(count)

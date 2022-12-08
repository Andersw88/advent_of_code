import json
import numpy

input = numpy.genfromtxt('input.csv', delimiter=',', dtype='unicode')

colums = len(input)
rows = len(input[0])
total_size = colums * rows

array = numpy.zeros(total_size, dtype=int)
visible_trees = numpy.zeros(total_size, dtype=bool)

for i, row in enumerate(input):
    for j, c in enumerate(row):
        array[j + i * rows] = int(c)

for i in range(rows):
    tallest_tree = -1
    for j in range(colums):
        index = j + i * rows
        tree_height = array[index]
        if tree_height > tallest_tree:
            visible_trees[index] = True
            tallest_tree = tree_height

    tallest_tree = -1
    for j in range(colums)[::-1]:
        index = j + i * rows
        tree_height = array[index]
        if tree_height > tallest_tree:
            visible_trees[index] = True
            tallest_tree = tree_height

for i in range(rows):
    tallest_tree = -1
    for j in range(colums):
        index = i + j * rows
        tree_height = array[index]
        if tree_height > tallest_tree:
            visible_trees[index] = True
            tallest_tree = tree_height

    tallest_tree = -1
    for j in range(colums)[::-1]:
        index = i + j * rows
        tree_height = array[index]
        if tree_height > tallest_tree:
            visible_trees[index] = True
            tallest_tree = tree_height


array = array.reshape((rows, colums))
visible_trees_square = visible_trees.reshape((rows, colums))

count = visible_trees.tolist().count(True)

print(array)

print(visible_trees_square)

print(count)

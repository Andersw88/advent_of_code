import numpy
import sys

input = numpy.genfromtxt('input.csv', delimiter=',', dtype='unicode')

columns = len(input)
rows = len(input[0])
total_size = columns * rows

array = numpy.zeros(total_size, dtype=int)
visible_trees = numpy.ones(total_size, dtype=int)

for i, row in enumerate(input):
    for j, c in enumerate(row):
        array[j + i * rows] = int(c)

for m in range(rows):
    for n in range(columns):
        current_tree = n + m * rows
        current_tree_height = array[current_tree]

        i = m
        number_of_trees = 0
        for j in range(n)[::-1]:
            index = j + i * rows
            tree_height = array[index]
            number_of_trees += 1
            if tree_height >= current_tree_height:
                break
        visible_trees[current_tree] *= number_of_trees

        number_of_trees = 0
        for j in range(n + 1, columns):
            index = j + i * rows
            tree_height = array[index]
            number_of_trees += 1
            if tree_height >= current_tree_height:
                break
        visible_trees[current_tree] *= number_of_trees

        i = n
        number_of_trees = 0
        for j in range(m)[::-1]:
            index = i + j * rows
            tree_height = array[index]
            number_of_trees += 1
            if tree_height >= current_tree_height:
                break
        visible_trees[current_tree] *= number_of_trees

        number_of_trees = 0
        for j in range(m + 1, columns):
            index = i + j * rows
            tree_height = array[index]
            number_of_trees += 1
            if tree_height >= current_tree_height:
                break
        visible_trees[current_tree] *= number_of_trees


array = array.reshape((rows, columns))
visible_trees_square = visible_trees.reshape((rows, columns))

# numpy.set_printoptions(threshold=sys.maxsize, linewidth=100000)
# print(visible_trees_square)

max_score = max(visible_trees)
print(max_score)

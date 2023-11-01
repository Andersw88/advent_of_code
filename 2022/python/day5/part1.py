
with open('input.csv') as file:
    input = [line for line in file]

number_of_stacks = 0

for i, row in enumerate(input):
    if row.strip().startswith("1"):
        number_of_stacks = int(row.strip()[-1])
        lowest_row = i - 1
        break

stacks = [[] for _ in range(number_of_stacks)]

for row in input[lowest_row::-1]:
    if row.strip().startswith("1"):
        break
    for i in range(int(len(row) / 4)):
        column = row[i * 4: i * 4 + 4]
        if column != "    ":
            block = column[1]
            stacks[i].append(block)

print(stacks, "\n")

for row in input[lowest_row + 3:]:
    columns = row.split()
    moves = int(columns[1])
    fromStack = int(columns[3]) - 1
    toStack = int(columns[5]) - 1
    for i in range(moves):
        stacks[toStack].append(stacks[fromStack].pop())


string = ''.join([x[-1] for x in stacks])

print(string)

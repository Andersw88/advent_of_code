import json

with open('input.csv') as file:
    input = [line for line in file]

root = {'/':{'folder_name':"/"}}
folder_stack = [root['/']]
folder_sizes = {}

for row in input:
    split = row.split()
    if split[0] == "$":
        command = split[1]
        if command == 'ls':
            pass
        if command == 'cd':
            folder = split[2]
            if folder == "/":
                folder_stack = [root['/']]
            elif folder == '..':
                folder_stack.pop()
            else:
                folder_stack.append(folder_stack[-1][folder] )
    else:
        if split[0] == 'dir':
            folder = split[1]
            folder_stack[-1][folder] = {'folder_name':folder_stack[-1]['folder_name'] + "." + folder}
        else:
            file_size = int(split[0])
            file_name = split[1]

            for folder in folder_stack[::-1]:
                if 'size_in_folder' in folder:
                    folder['size_in_folder'] += file_size
                else:
                    folder['size_in_folder'] = file_size
                folder_sizes[folder['folder_name']] = folder['size_in_folder']

            folder_stack[-1][file_name] = file_size

use_size = root['/']['size_in_folder']
total_size = 70000000
needed_size = 30000000
space_needed = -(total_size - needed_size - use_size)

min_folder = use_size
for key, value in folder_sizes.items():
    if value < min_folder and value > space_needed:
        min_folder = value

print('min_folder', min_folder)




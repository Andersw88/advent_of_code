import numpy as np
import parse
input = np.genfromtxt('input.csv',delimiter='\n', dtype='unicode', comments=None)

# [1518-11-01 00:00] Guard #10 begins shift
# [1518-11-01 00:05] falls asleep
# [1518-11-01 00:25] wakes up

parse_string = '[{}-{}-{} {}:{}] Guard #{:d} begins shift'
parse_string2 = '[{}-{}-{} {}:{}] {}'

time_array = []


for line in input:
    result = parse.parse(parse_string, line)
    minute = 0
    if result == None:
        result = parse.parse(parse_string2, line)
        minute = int(result[4])
    time_array.append([int(result[0]+result[1]+result[2]+result[3]+result[4]),result[5], minute])



print(time_array)

guard_dict = {}
currentGuard = None
sleep_time = None
for line in sorted(time_array, key=lambda x: x[0]):
    # result = parse.parse(parse_string, line)
    if isinstance(line[1], int):
        currentGuard = line[1]
        if not currentGuard in guard_dict:
            guard_dict[currentGuard] = [0, np.zeros(60, dtype=int)]
    else:
        if line[1] == 'falls asleep':
            sleep_time = line[2]
        elif line[1] =='wakes up':
            wake_time = line[2]
            guard_dict[currentGuard][0] +=  wake_time - sleep_time
            guard_dict[currentGuard][1][sleep_time:wake_time] += 1
        else:
            raise NameError(line[1])

# max_guard = max(guard_dict, key=lambda x: guard_dict[x][0])

# max_guard_minute = np.argmax(guard_dict[max_guard][1])

max_guard = max(guard_dict, key=lambda x: np.max(guard_dict[x][1]))

max_guard_minute = np.argmax(guard_dict[max_guard][1])

print(max_guard_minute, max_guard, guard_dict[max_guard])

print(max_guard_minute * max_guard)

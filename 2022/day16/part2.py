import numpy
import sys
import parse
import functools
import networkx as nx

graph = nx.Graph()

with open('input.csv') as file:
    input = [line for line in file]
numpy.set_printoptions(threshold=sys.maxsize, linewidth=800)

open_valves = {}
valves = {}


class Valve:
    def __init__(self, name, rate, other_valves):
        self.name = name
        self.rate = rate


for row in input:
    split_row = row.split()

    name = split_row[1]
    rate = int(split_row[4][5:-1])

    other_valves = [valve.strip(',') for valve in split_row[9:]]

    valves[name] = Valve(name, rate, other_valves)
    if rate != 0:
        open_valves[name] = valves[name]

    graph.add_node(name)
    for valve_connection in other_valves:
        if graph.has_node(valve_connection):
            graph.add_node(valve_connection)
        graph.add_edge(name, valve_connection)

shortest_paths = dict(nx.all_pairs_dijkstra_path_length(graph))
max_released = 0


class Agent:
    def __init__(self, position, time_remaining, pressure_released=0, flow_rate=0):
        self.time_remaining = time_remaining
        self.position = position

    def __hash__(self):
        return hash((self.time_remaining, self.position))

    def __lt__(self, other):

        return (self.time_remaining, self.position) < (other.time_remaining, other.position)

    def __repr__(self):
        return str(self.position)

    def __eq__(self, other):
        return (self.time_remaining, self.position) == (other.time_remaining, other.position)


@functools.lru_cache(maxsize=None)
def step(args: tuple):

    agents = list(args[0])
    remaining_open_valves = args[1]
    max_this_instance = 0

    for i, agent_ref in enumerate(agents):
        for open_valve in remaining_open_valves:

            length_to_valve = shortest_paths[agent_ref.position][open_valve]
            tempTimeRemainging = agent_ref.time_remaining - \
                (length_to_valve + 1)
            if tempTimeRemainging <= 0:
                continue
            agent = Agent(
                agent_ref.position, agent_ref.time_remaining)
            new_agents = [agent, agents[1 if i == 0 else 0]]
            # new_agents = [agent]
            new_agents.sort()

            agent.time_remaining = tempTimeRemainging
            open_valve2 = list(remaining_open_valves)
            open_valve2.remove(open_valve)
            pressure_released = valves[open_valve].rate * agent.time_remaining

            agent.position = open_valve

            passed_args = (tuple(new_agents), tuple(open_valve2))

            pressure_released += step(passed_args)
            max_this_instance = max(max_this_instance, pressure_released)

    return max_this_instance


remaining_open_valves = tuple(open_valves.keys())
start = 'AA'

time_remaining = 26
agents = (Agent(start, time_remaining), Agent(start, time_remaining))

# time_remaining = 30
# agents = (Agent(start, time_remaining),)

print(step((agents, remaining_open_valves)))

info = step.cache_info()
print(info)

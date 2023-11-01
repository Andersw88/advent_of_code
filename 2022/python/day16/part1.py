import numpy
import math
import json
import sys
import parse

import networkx as nx
import matplotlib.pyplot as plt

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

shortest_paths = dict(nx.all_pairs_dijkstra(graph))

max_released = 0


def step(start: str, remaining_open_valve: set, time_remaining, flow_rate=0, pressure_released=0, followed_path=[], pressure_history=[]):
    global max_released

    if len(remaining_open_valve) == 0:
        pressure_released2 = pressure_released + time_remaining * flow_rate
        if pressure_released2 > max_released:
            print(pressure_released2, followed_path,
                  time_remaining, flow_rate, pressure_history)
            max_released = pressure_released2

    for open_valve in remaining_open_valve:

        length_to_valve = shortest_paths[start][0][open_valve]

        time_remaining2 = time_remaining - (length_to_valve + 1)

        if time_remaining2 < 0:
            pressure_released2 = pressure_released + time_remaining * flow_rate

            if pressure_released2 > max_released:
                print(pressure_released2, followed_path,
                      time_remaining, flow_rate, pressure_history)
                max_released = pressure_released2
            continue
        open_valve2 = remaining_open_valve.copy()
        open_valve2.remove(open_valve)

        followed_path2 = followed_path.copy()
        followed_path2.append(open_valve)

        pressure_released2 = pressure_released + \
            flow_rate * (length_to_valve + 1)
        new_rate = flow_rate + valves[open_valve].rate

        pressure_history2 = pressure_history.copy()
        pressure_history2.append(pressure_released2)

        step(open_valve, open_valve2, time_remaining2,
             new_rate, pressure_released2, followed_path2, pressure_history2)


remaining_open_valves = set(open_valves.keys())
start = 'AA'
time_remaining = 30

step(start, remaining_open_valves, time_remaining)

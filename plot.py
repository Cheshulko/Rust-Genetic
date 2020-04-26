#!/usr/bin/python

import sys
import matplotlib.pyplot as plt
import networkx as nx

# 1 arg - cities path
# 2 arg - output path

plt.figure(figsize=(12, 6))

cities_path = str(sys.argv[1])
output_path = str(sys.argv[2])

finput = open(cities_path, newline='')
lines = finput.readlines()

cities = []

for line in lines:
    splited = line.split(',')
    cityIndex = int(splited[0])
    cityX = float(splited[1])
    cityY = float(splited[2])

    cities.append((cityX, cityY))

finput = open(output_path, newline='')
readed_lines = finput.readlines()

progress = []
lines = []
G=nx.Graph()

for line in readed_lines:
    fit = float(line.split(' ')[2].replace(",", ""))
    lines.append(line)
    progress.append(1 / fit)

def prepare_line(i):
    splited_last_line = lines[i].split(' ')
    cnt = int(splited_last_line[1].replace(",", ""))
    return (splited_last_line, cnt)

def build_graph(cnt, splited_last_line):
    edges = []
    for i in range(cnt):
        frome = int(splited_last_line[4 + i].replace(",", ""))
        toe = int(splited_last_line[5 + i].replace(",", ""))
        edges.append((frome, toe))
    return edges

def draw_graph():
    G.clear()
    for x in enumerate(cities):
        index = x[0]
        point = x[1]
        G.add_node(index, pos=point)

    for edge in edges:
        G.add_edge(*edge)

generations = len(readed_lines)
for i in range(generations):
    splitedLastLine, cnt = prepare_line(i)
    edges = build_graph(cnt, splitedLastLine)
    draw_graph()

    plt.clf()
    plt.suptitle('Generation {} of {}'.format(i+1, generations))

    plt.subplot(121)
    plt.xlim(0, generations)
    plt.ylim(0, progress[0])
    plt.ylabel('Distance')
    plt.xlabel('Generation')
    plt.plot(progress[0:i])

    plt.subplot(122)
    pos = nx.get_node_attributes(G,'pos')
    nx.draw(G, pos=pos, node_size=20)
    plt.pause(0.001)

plt.show()

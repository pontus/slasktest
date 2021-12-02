#!/usr/bin/env python

depth = 0
distance = 0
aim = 0

with open('input.txt') as f:
    for line in f:
        verb, arg = line.split()

        if verb == 'forward':
            distance = distance + int(arg)
            depth = depth + aim * int(arg)
        if verb == 'up':
            aim = max(aim - int(arg), 0)
        if verb == 'down':
            aim = aim + int(arg)

print(distance*depth)
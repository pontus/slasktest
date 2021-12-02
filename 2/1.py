#!/usr/bin/env python

depth = 0
distance = 0

with open('input.txt') as f:
    for line in f:
        verb, arg = line.split()

        if verb == 'forward':
            distance = distance + int(arg)
        if verb == 'up':
            depth = max(depth - int(arg), 0)
        if verb == 'down':
            depth = depth + int(arg)

print(distance*depth)
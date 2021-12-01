#!/usr/bin/env python

old_depth = 0
count = 0
with open('input.txt') as f:
    for line in f:
        depth = int(line)
        if depth > old_depth:
            count += 1
        old_depth = depth

print(count-1)
#!/usr/bin/env python

from curses import window


window = []
old_ws = 0
count = 0

with open('input.txt') as f:
    for line in f:
        depth = int(line)

        if (len(window) >= 3):
            ws = sum(window[:3])

            if ws > old_ws:
                count += 1
            old_ws = ws

        if (len(window) > 2):
            window = window[1:]

        window.append(depth)


print(count)
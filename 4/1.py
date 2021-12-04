#!/usr/bin/env python


def read_bords(f):
    done = False
    boards = []
    while not done:
        f.readline()

        thisboard = []
        for p in range(5):
            s = f.readline()
            if not s:
                done = True
                break

            l = [int(x) for x in s.split()]
            thisboard.append(l)
        boards.append(thisboard)
    return boards


def mark_number(n, boards):
    for b in boards:
        for l in b:
            if n in l:
                l[l.index(n)] = -1


def check_boards_for_win(boards):
    for b in boards:
        for p in b:
            if sum(p) == -5:
                return b
        for p in range(5):
            if sum(l[p] for l in b) == -5:
                return b
    return None


def sum_unmarked(b):
    l = 0
    for p in b:
        l = l + sum([x if x != -1 else 0 for x in p])
    return l


with open("input.txt") as f:

    numbers = [int(x) for x in f.readline().split(",")]
    boards = read_bords(f)

    for n in numbers:
        mark_number(n, boards)
        b = check_boards_for_win(boards)
        if b:
            print(n)
            print(b)
            print(n * sum_unmarked(b))
            break

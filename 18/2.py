#!/usr/bin/env python

import math
import time
import copy


def magnitude(s):
    l = 0
    if s["pair_left"]:
        l = magnitude(s["pair_left"][0])
    else:
        l = s["regular_left"]

    r = 0
    if s["pair_right"]:
        r = magnitude(s["pair_right"][0])
    else:
        r = s["regular_right"]
    return 3 * l + 2 * r


def to_s(s):
    rs = "["
    if s["pair_left"]:
        rs += to_s(s["pair_left"][0])
    else:
        rs += str(s["regular_left"])
    rs += ", "
    if s["pair_right"]:
        rs += to_s(s["pair_right"][0])
    else:
        rs += str(s["regular_right"])
    rs += "]"
    return rs


def find_depth(s, depth):

    l = []
    if not s["pair_left"] and not s["pair_right"]:
        return [(depth, s)]

    if s["pair_left"]:
        # print(s["pair_left"])
        l = l + find_depth(s["pair_left"][0], depth + 1)

    l = l + [(depth, s)]
    if s["pair_right"]:
        l = l + find_depth(s["pair_right"][0], depth + 1)

    return l


def parent(s, child):
    if s["pair_left"] and s["pair_left"][0] == child:
        return s
    if s["pair_right"] and s["pair_right"][0] == child:
        return s

    if s["pair_left"]:
        l = parent(s["pair_left"][0], child)
        if l:
            return l

    if s["pair_right"]:
        l = parent(s["pair_right"][0], child)
        if l:
            return l

    return None


def explode(s):
    d = find_depth(s, 0)

    replace = None
    replaceid = None

    for i in range(len(d)):
        if d[i][0] >= 4:
            # print("Will replace")
            # print(to_s(d[i][1]))
            left = d[i][1]["regular_left"]
            right = d[i][1]["regular_right"]

            replace = d[i][1]
            replaceid = i
            break

    if not replace:
        return (s, False)

    # print("Parents")
    p = parent(s, replace)

    if p["pair_left"] and p["pair_left"][0] == replace:
        # We are replacing the left child
        p["regular_left"] = 0
        p["pair_left"] = []

        if not p["pair_right"]:
            p["regular_right"] += right
        else:
            # Go down the left part of the right child
            x = p["pair_right"][0]
            while x["pair_left"]:
                x = x["pair_left"][0]
            x["regular_left"] += right

        # print("Will go upwards "+to_s(p))

        x = p
        y = parent(s, p)
        while True:
            # While we're leftmost
            if not y:
                # Reached the top and we were the leftmost
                # print("Bailing out "+to_s(x))

                return s, True

            # Go upwards while we're leftmost
            if y["pair_left"] and y["pair_left"][0] == x:
                x = y
                y = parent(s, y)
                continue

            # print("Gone upwards and found "+to_s(y))
            # We're not leftmost
            if not y["pair_left"]:
                # Regular number
                y["regular_left"] += left
                return s, True
            else:
                # Pair to the left, go down as long as we have a pair to the right
                x = y["pair_left"][0]
                while x["pair_right"]:
                    x = x["pair_right"][0]
                x["regular_right"] += left
                return s, True

    if p["pair_right"] and p["pair_right"][0] == replace:
        p["regular_right"] = 0
        p["pair_right"] = []

        if not p["pair_left"]:
            p["regular_left"] += left
        else:
            x = p["pair_left"][0]
            # Go down the right part of the left child
            while x["pair_right"]:
                x = x["pair_right"][0]
            x["regular_right"] += left

        x = p
        y = parent(s, p)
        while True:
            # While we're rightmost
            if not y:
                # Reached the top and we were the rightmost
                return s, True

            # Go upwards while we're rightmost
            if y["pair_right"] and y["pair_right"][0] == x:
                x = y
                y = parent(s, y)
                continue

            # We're not rightmost
            if not y["pair_right"]:
                # Regular number
                y["regular_right"] += right
                return s, True
            else:
                # Pair to the left, go down as long as we have a pair to the right
                x = y["pair_right"][0]
                while x["pair_left"]:
                    x = x["pair_left"][0]
                x["regular_left"] += right
                return s, True

    return (s, False)


def split(s):
    rets = s

    if not s["pair_left"]:
        if s["regular_left"] > 9:
            s["pair_left"] = [
                {
                    "id": time.monotonic(),
                    "regular_left": math.floor(s["regular_left"] / 2),
                    "pair_left": [],
                    "regular_right": math.ceil(s["regular_left"] / 2),
                    "pair_right": [],
                }
            ]
            s["regular_left"] = 0
            return s, True
    else:
        subs, done = split(s["pair_left"][0])
        if done:
            s["pair_left"] = [subs]
            return s, True

    if not s["pair_right"]:
        if s["regular_right"] > 9:
            s["pair_right"] = [
                {
                    "id": time.monotonic(),
                    "regular_left": math.floor(s["regular_right"] / 2),
                    "pair_left": [],
                    "regular_right": math.ceil(s["regular_right"] / 2),
                    "pair_right": [],
                }
            ]
            s["regular_right"] = 0

            return s, True
    else:
        subs, done = split(s["pair_right"][0])
        if done:
            s["pair_right"] = [subs]
            return s, True

    return s, False


def parse_number(s):
    left = []
    right = []
    i = 0
    num = {
        "id": time.monotonic(),
        "regular_left": 0,
        "pair_left": left,
        "regular_right": 0,
        "pair_right": right,
    }

    if s[0] != "[":
        raise Exception("fel")

    if s[1] == "[":
        l, consumed = parse_number(s[1:])
        left.append(l)
        i += consumed + 3
    else:
        comma = s.find(",")
        num["regular_left"] = int(s[1:comma])
        i = comma + 1

    if s[i] == "[":
        r, consumed = parse_number(s[i:])
        right.append(r)
        i += consumed + 1
    else:
        bracket = s.find("]", i)
        num["regular_right"] = int(s[i:bracket])
        i = bracket

    return num, i


def reduce(s):
    # print("reducing" + to_s(s))
    change = True
    while change:
        s, change = explode(s)
        if change:
            # print("exploded " + to_s(s))
            continue
        s, change = split(s)
        if change:
            # print("split " + to_s(s))
            pass

    return s


def add(a, b):
    c = copy.deepcopy(a)
    d = copy.deepcopy(b)

    s = {
        "id": time.monotonic(),
        "regular_left": 0,
        "regular_right": 0,
        "pair_left": [c],
        "pair_right": [d],
    }
    # print("Add result : "+ to_s(s))
    x = reduce(s)
    return x


f = open("input.txt", "r")

s = "[[[[13, 7], [14, 12]], [[0, [10, 6]], 9]], [1, [[[9, 3], 9], [[9, 0], [0, 7]]]]]"

# a= parse_number(s.replace(" ",""))[0]  #print(to_s(reduce(a)))
# print(reduce(a))

# a= parse_number("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")[0]
# a = parse_number("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")[0]
# b =  reduce(a) #b = explode(a)[0]
# print(to_s(b))
# a = parse_number("[[[[0,7],4],[15,[0,13]]],[1,1]]")[0]
# b = split(a)[0]
# print(to_s(b))
# raise SystemExit

l = []
for line in f:
    n = parse_number(line)
    l.append(n[0])

maxm = 0

for i in range(len(l)):
    print("i: " + str(i) + "  :" + to_s(l[i]))

for p in range(len(l)):
    for q in range(len(l)):
        if p != q:
            print("")
            print(p)
            print(q)
            print(to_s(l[p]))
            print(to_s(l[q]))
            s = add(l[p], l[q])
            m = magnitude(s)
            print(m)
            if m > maxm:
                maxm = m

print(maxm)

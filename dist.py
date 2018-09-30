#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from collections import deque

def idx2xy(pos):
    return pos%8, pos//8

def xy2idx(x, y):
    return 8*y + x

def adjacent(pos):
    x, y = idx2xy(pos)
    for xx,yy in [(x+1,y), (x-1,y), (x,y+1), (x,y-1)]:
        xx %= 8
        yy %= 8
        yield xy2idx(xx,yy)

def dist_from(pos):
    res = [None] * 64

    que = deque()
    que.append(pos)
    res[pos] = 0

    while que:
        i = que.popleft()

        for j in adjacent(i):
            if res[j] is not None: continue
            que.append(j)
            res[j] = res[i] + 1

    return res

def main():
    dist = [dist_from(i) for i in range(64)]

    print("const DIST: [[u32; 64]; 64] = [")
    for i in range(64):
        print("[{}],".format(",".join(map(str,dist[i]))))
    print("];")

if __name__ == "__main__": main()

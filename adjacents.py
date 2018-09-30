#!/usr/bin/env python3
# -*- coding: utf-8 -*-

def idx2xy(pos):
    return pos%8, pos//8

def main():
    print("const ADJACENTS: [&'static[u8]; 64] = [")
    for i in range(64):
        adj = []
        x, y = idx2xy(i)
        if 1 <= y: adj.append(i-8)
        if 1 <= x: adj.append(i-1)
        if x <= 6: adj.append(i+1)
        if y <= 6: adj.append(i+8)
        print("&[{}]".format(", ".join(map(str, adj))), end="")
        if i%8 == 7:
            print(",")
        else:
            print(", ", end="")
    print("];")

    print("const ADJACENTS_H: [&'static[u8]; 64] = [")
    for i in range(64):
        adj = []
        x, _ = idx2xy(i)
        if 1 <= x: adj.append(i-1)
        if x <= 6: adj.append(i+1)
        print("&[{}]".format(", ".join(map(str, adj))), end="")
        if i%8 == 7:
            print(",")
        else:
            print(", ", end="")
    print("];")

    print("const ADJACENTS_V: [&'static[u8]; 64] = [")
    for i in range(64):
        adj = []
        _, y = idx2xy(i)
        if 1 <= y: adj.append(i-8)
        if y <= 6: adj.append(i+8)
        print("&[{}]".format(", ".join(map(str, adj))), end="")
        if i%8 == 7:
            print(",")
        else:
            print(", ", end="")
    print("];")

if __name__ == "__main__": main()

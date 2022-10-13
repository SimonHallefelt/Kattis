#fel

from operator import index
from re import A

#       a    b    c    d    e    f    g    h    i    j    k    l    m    n    o    p    q    r    s    t    u    v    w    x    y    z
pos = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']
n = int(input())

for i in range(n):
    s = input()
    last = int(pos.index(s[0]))
    moves = 0
    for j in s:
        print(moves)
        a = int(pos.index(j))
        if a == last:
            continue
        else:
            if abs(a-last) > 13:
                moves += 26 - abs(a-last)
            else:
                moves += abs(a-last)
            last = a
    print(moves)
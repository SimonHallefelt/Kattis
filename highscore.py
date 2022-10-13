from operator import index

#       a    b    c    d    e    f    g    h    i    j    k    l    m    n    o    p    q    r    s    t    u    v    w    x    y    z
pos = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']
#       0    1    2    3    4    5    6    7    8    9    10   11   12   13   12   11   10   9    8    7    6    5    4    3    2    1
n = int(input())

for i in range(n):
    s = input()
    l = len(s)
    last = int(pos.index(s[0]))
    moves = 0
    steps = l-1
    stepmap = []
    for j in s:
        a = int(pos.index(j))
        if a == 0:
            stepmap.append(0)
        else:
            if a > 13:
                moves += 26 - a
            else:
                moves += a
            last = a
            stepmap.append(1)

    last = 0
    stepmap[0] = 0
    numberOfA = 0
    if sum(stepmap) == 0:
        print(moves)
    elif sum(stepmap) != l-1:
        for j in range(l*2):
            temp = j%l
            temp2 = 0
            if stepmap[temp] == 1:
                if last != 0 and numberOfA != 0:
                    if temp > last:
                        if l-temp < last:
                            temp2 = (l-temp)*2 + last
                        else:
                            temp2 = (last)*2 + (l-temp)
                    else:
                        if l-last > temp:
                            temp2 = last
                        else:
                            temp2 = l-temp
                    if temp2 < steps:
                        steps = temp2
                last = temp
                numberOfA = 0
            else:
                numberOfA += 1
    print(moves + steps)
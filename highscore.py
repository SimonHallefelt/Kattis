from operator import index

#       a    b    c    d    e    f    g    h    i    j    k    l    m    n    o    p    q    r    s    t    u    v    w    x    y    z
pos = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']
#       0    1    2    3    4    5    6    7    8    9    10   11   12   13   12   11   10   9    8    7    6    5    4    3    2    1
n = int(input())

for i in range(n):
    s = input()
    cost = 0
    stepmap = []
    for j in s:
        a = int(pos.index(j))
        if a == 0:
            stepmap.append(0)
        else:
            if a > 13:
                cost += 26 - a
            else:
                cost += a
            stepmap.append(1)

    l = len(s)
    steps = l-1
    last = 0
    stepmap[0] = 0
    hasBeenA = False
    if sum(stepmap) == 0:
        print(cost)
    elif sum(stepmap) == l-1:
        print(cost + l-1)
    else:
        for j in range(l*2):
            temp = j%l
            temp2 = 0
            if stepmap[temp] == 1:
                if last != 0 and hasBeenA:
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
                hasBeenA = False
            else:
                hasBeenA = True
        print(cost + steps)
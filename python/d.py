from itertools import permutations

matrix = []
for i in range(0, 7):
    matrix.append(input())
d, h = map(int, input().split())

pro = []
l = [0,1,2,3,4,5,6]
for a in permutations(l, d):
    list = []
    for i in range(0,24):
        val = 0.0
        for b in a:
            if matrix[b][i] == '.':
                val += 1.0
        list.append(val)
    list.sort(reverse=True)
    val = 0.0
    for i in range(0,h):
        val += list[i]
    pro.append(val)

print(pro)
print(pro[0] / (d*h))
g = int(input())

list = []
for i in range(g):
    list.append(int(input()))


for i in range(len(list), 1000000):
    s = set()
    for j in range(len(list)):
        s.add(list[j] % i)
    if len(s) == len(list):
        print(i)
        break
    
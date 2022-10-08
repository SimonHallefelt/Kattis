n, m = map(int, input().split())

list = []
temp = 1
sne = 0

while m >= temp:
    list.append(temp)
    temp *= 2

list.reverse()
for i in range(len(list)):
    if m >= list[i]:
        m -= list[i]
        sne += 1

print(sne)
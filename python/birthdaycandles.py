n,m,k = map(int , input().split())

candles = []
for i in range(0,n):
    temp = [int(j) for j in input().split()]
    temp.sort()
    candles.append(temp)

number = 0
b = False
for i in range(0,m):
    temp = []
    for c in candles:
        temp.append(c[i])
    temp.sort()
    for t in temp:
        if k >= t:
            number += 1
            k -= t
        else:
            b = True
            break
    if b:
        break

print(number)
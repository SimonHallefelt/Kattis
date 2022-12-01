n = int(input())
s = input().split()

total = 0
plats = []
s2 = []

for i in range(n+1):
    plats.append(0)
    s2.append(0)

for i in range(n):
    plats[int(s[i])] = i+1
    s2[i+1] = int(s[i])

for i in range(1, n+1):
    if not s2[i] == i:
        total += 1
        s2[plats[i]] = s2[i]
        plats[s2[i]] = plats[i]

print(total)
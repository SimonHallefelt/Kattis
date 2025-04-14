n,m = map(int , input().split())

d = {}
for i in range(0,n):
    s = input()
    temp = s.split()
    ss = temp[0][0]+temp[1][0]
    if ss in d:
        d[ss] = "ambiguous"
    else:
        d[ss] = s

for i in range(0,m):
    s = input()
    if s in d:
        print(d[s])
    else:
        print("nobody")
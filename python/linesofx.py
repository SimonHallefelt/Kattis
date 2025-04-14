n = int(input())

m = []
for i in range(0,n):
    m.append([c for c in input()])

solvers = []
empty = 0
hasPerfect = False
b1 = 0
b2 = 0
for i in range(0,n):
    if b1 != -1:
        if m[i][i] == "o":
            b1 = -1
        elif m[i][i] == '.':
            b1 += 1

    if b2 != -1:
        if m[i][n-i-1] == "o":
            b2 = -1
        elif m[i][n-i-1] == '.':
            b2 += 1

    b3 = 0
    b4 = 0
    for j in range(0,n):
        if m[i][j] == '.':
            empty += 1
        
        if b3 != -1:
            if m[i][j] == "o":
                b3 = -1
            elif m[i][j] == '.':
                b3 += 1

        if b4 != -1:
            if m[j][i] == "o":
                b4 = -1
            elif m[j][i] == '.':
                b4 += 1
    
    if b3 == 0 or b4 == 0:
        hasPerfect = True
    
    if b3 > 0:
        solvers.append(b3)
    if b4 > 0:
        solvers.append(b4)


if b1 == 0 or b2 == 0 or hasPerfect:
    print(2**empty)
    exit()

if b1 > 0:
    solvers.append(b1)
if b2 > 0:
    solvers.append(b2)

if len(solvers) == 0:
    print(0)
    exit()

sum = 0
for i in range(0, len(solvers)):
    s = solvers[i]
    temp = 2**(empty-s-1)
    sum += temp
print(sum)
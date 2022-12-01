import math


a,b = map(int, input().split())

jonnaBalls = []
opposingBalls = []


for j in range(6):
    x,y = map(int, input().split())
    d = math.sqrt((x-a)**2 + (y-b)**2)
    jonnaBalls.append(d)

for j in range(6):
    x,y = map(int, input().split())
    d = math.sqrt((x-a)**2 + (y-b)**2)
    opposingBalls.append(d)

jonnaBalls.sort()
opposingBalls.sort()

points = 0
if jonnaBalls[0] == opposingBalls[0]:
    print("TIE")
elif jonnaBalls[0] < opposingBalls[0]:
    print("JONNA")
    for i in range(6):
        if jonnaBalls[i] < opposingBalls[0]:
            points += 1
    print(points)
else:
    print("OPPONENTS")
    for i in range(6):
        if jonnaBalls[0] > opposingBalls[i]:
            points += 1
    print(points)


import math

n, v  = map(int, input().split())

if v >= 0 and v <= 180:
    print("safe")
else:
    if v < 270:
        v = v - 180
    else:
        v = 360 - v
    print(int(n/math.sin(math.radians(v))))
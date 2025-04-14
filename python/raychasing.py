x_1,x_2,y_1,y_2 = map(int , input().split())
x1,y1,x2,y2 = map(int , input().split())

if x1 == x2:
    if y1 < y2: # travel up
        print("top")
    else:
        print("bottom")
    exit()
dx = (y2-y1)/(x2-x1)

if x1 < x2: # travel right
    y = (x_2-x1) * dx + y1
    if y == y_1:
        print("bottom-right")
    elif y == y_2:
        print("top-right")
    elif y < y_1:
        print("bottom")
    elif y > y_2:
        print("top")
    else:
        print("right")
else:
    y = (x_1-x1) * dx + y1
    if y == y_1:
        print("bottom-left")
    elif y == y_2:
        print("top-left")
    elif y < y_1:
        print("bottom")
    elif y > y_2:
        print("top")
    else:
        print("left")


max1 = 0
max2 = 0
max3 = 0


last = 0
while(True):
    s = input()
    if s == "":
        last = 0
    else:
        last += int(s)
        if last > max3:
            max3 = last
            if max3 > max2:
                temp = 0
                temp = max2
                max2 = max3
                max3 = temp
                if max2 > max1:
                    temp = 0
                    temp = max1
                    max1 = max2
                    max2 = temp
        print(max1, max2, max3)
        print(max1+max2+max3)
n = int(input())
word = input()

a = int(0)
while a <= n:
    if n-a > 1:
        print(str(n-a) + " bottles of " + word + " on the wall, " + str(n-a) + " bottles of " + word + ".")
        if n-a-1 > 1:
            print("Take one down, pass it around, " + str(n-a-1) + " bottles of " + word + " on the wall.") 
        else:
            print("Take one down, pass it around, 1 bottle of " + word + " on the wall.")
    else:
        print("1 bottle of " + word + " on the wall, 1 bottle of " + word + ".")
        print("Take it down, pass it around, no more bottles of " + word + ".")
        break
    a += 1 

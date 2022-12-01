N = int(input())
if N < 149:
    print(99)
elif N % 100 >= 49:
    print((N//100)*100 + 99)
else:
    print((N//100)*100 - 1)
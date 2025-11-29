n = int(input())
m = -1
if n != 0:
    m = n - (abs(n)/n)
if m == 0:
    m -= 2
print(int(m), int(n-m))
n = int(input())
g = [int(x) for x in input().split()]
k = [int(x) for x in input().split()]
g.sort()
k.sort()
p = 1000000000000059

a = [0]*n
b = [0]*n
for i in range(n):
    a[i] = (g[(i+1)%n] - g[(i)%n])%360000
    b[i] = (k[(i+1)%n] - k[(i)%n])%360000
        
d = 0
c = 0
for i in range(n):
    c = (c*256+a[i])%p 
    d = (d*256+b[i])%p 

pw = pow(256, n-1, p)
for i in range(0, n):
    if c == d:
        print("possible")
        exit()
    c = ((c-a[i] * pw)*256 + a[i])%p

print("impossible")


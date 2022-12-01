#fel

n,m = map(int , input().split())
toEnd = set()
graf = {}

for i in range(1, n+1):
    toEnd.add(i)
    graf[i] = []

for i in range(m):
    a, b = map(int , input().split())
    graf[a].append(b)
    graf[b].append(a)

bool bfs(s):
    

for i in range(1, n+1):
    if not bfs(i):
        print("Impossible")
        exit()

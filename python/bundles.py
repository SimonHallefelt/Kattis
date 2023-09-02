class node:
    def __init__(self, price, contai) -> None:
        self.c = contai
        self.price = price
        self.children = []

    def insert(self, l):
        t = False
        for r in self.children:
            if l.c[0] in r.c:
                r.insert(n)
                t = True
                break 
        if not t:
            self.children.append(l)

    def cost(self):
        h = []
        for g in self.children:
            h += g.c
        if len(self.c) == len(h):
            tot = 0
            for g in self.children:
                tot += g.cost()
            return min(tot, self.price)
        return self.price
    
    def __str__(self) -> str:
        return str(self.c) + str(self.price) + str(self.children)


for _ in range(int(input())):
    n, m = map(int, input().split())
    nodes = []
    for _ in range(m):
        k = [int(c) for c in input().split()]
        price = k[0]
        val = k[2:]
        nodes.append(node(price, val))
    nodes.sort(key=lambda a: len(a.c)*1000+a.price, reverse=True)
    res = []
    for n in nodes:
        t = False
        for r in res:
            if n.c[0] in r.c:
                r.insert(n)
                t = True
                break 
        if not t:
            res.append(n)
    s = 0
    for n in res:
        s += n.cost()
    print(s)
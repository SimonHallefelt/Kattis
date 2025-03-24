def kosaraju(adj):
    n = len(adj)
    scc = []
    part_of = [0] * n
    stack = []

    def dfs(adj, order, first_iteration):
        visited = [False] * n
        dfs_stack = []
        for node in order:
            if not visited[node]:
                visited[node] = True
                dfs_stack.append((node, 0))
                if not first_iteration:
                    scc.append([])
                while dfs_stack:
                    (node, next) = dfs_stack.pop()
                    visited[node] = True
                    if next == len(adj[node]):
                        if first_iteration:
                            stack.append(node)
                        else:
                            part_of[node] = len(scc) - 1
                            scc[-1].append(node)
                    else:
                        dfs_stack.append((node, next + 1))
                        if not visited[adj[node][next]]:
                            dfs_stack.append((adj[node][next], 0))

    dfs(adj, range(n), True)

    adj_rev = [[] for _ in range(n)]
    for (a, adj) in enumerate(adj):
        for b in adj:
            adj_rev[b].append(a)

    dfs(adj_rev, reversed(stack), False)

    is_dag = n == len(scc)
    return (scc, part_of, is_dag)


class Sat:
    
    def __init__(self, n):
        self.n = n
        self.adj = [[] for _ in range(2 * n)] # not x -> x + n

    
    def add_edge(self, u, v):
        self.adj[u].append(v)
    
    
    def add_or(self, a: int, an: bool, b: int, bn: bool):
        self.add_edge(a + (self.n if an else 0), b + (0 if bn else self.n))
        self.add_edge(b + (self.n if bn else 0), a + (0 if an else self.n))
    
    
    def set_true(self, a: int, an: bool):
        self.add_or(a, an, a, an)
    
    
    def add_xor(self, a: int, an: bool, b: int, bn: bool):
        self.add_or(a, an, b, bn)
        self.add_or(a, not an, b, not bn)
    
    
    def evaluate(self):
        _, part_of, _ = kosaraju(self.adj)
        if any(part_of[i] == part_of[i + self.n] for i in range(self.n)):
            return None
        return [part_of[i] > part_of[i + self.n] for i in range(self.n)]

"""
    (a OR not b) AND (not a OR b) AND (a OR b) AND (not a AND c)
"""
sat = Sat(3)
sat.add_or(0, True, 1, False)
sat.add_or(0, False, 1, True)
sat.add_or(0, True, 1, True)
sat.add_or(0, False, 2, True)
print(sat.evaluate())
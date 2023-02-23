def kosaraju(adj):
    n = len(adj)
    sccs = []
    part_of = [0] * n

    # Step 1
    stack = []
    visited = [False] * n
    def dfs1(node):
        if visited[node]:
            return
        visited[node] = True
        for next in adj[node]:
            dfs1(next)
        stack.append(node)

    for node in range(n):
        dfs1(node)

    # step 1.5
    adj_rev = [[] for _ in range(n)]
    for i in range(n):
        for other in adj[i]:
            adj_rev[other].append(i)

    # step 2
    visited = [False] * n
    def dfs2(node):
        visited[node] = True
        sccs[-1].append(node)
        part_of[node] = len(sccs) - 1
        for next in adj_rev[node]:
            if not visited[next]:
                dfs2(next)

    for node in reversed(stack):
        if not visited[node]:
            sccs.append([])
            dfs2(node)

    return (sccs, part_of, len(sccs) == n)
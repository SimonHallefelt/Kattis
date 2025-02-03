from heapq import heapify, heappop, heappush
n = int(input())

points = []
pokemon = []
pokemon_dict = {}
for i in range(n):
    line = input().split()
    points.append(list(map(int, line[:2])))
    poke = line[2]
    if poke not in pokemon_dict:
        pokemon_dict[poke] = len(pokemon_dict)
    
    pokemon.append(pokemon_dict[poke])
    

n_pokemon = len(pokemon_dict)

# state: (node, bitmask_taken_pokemons)
dist = [[10**15 for _ in range(1 << n_pokemon)] for _ in range(n)]

def manhattan(a, b):
    return abs(a[0] - b[0]) + abs(a[1] - b[1])

# dist[node][bitmask]
q = []
for i in range(n):
    heappush(q , (
        manhattan(points[i], (0, 0)),
        i,
        1 << pokemon[i]
    ))
    dist[i][1 << pokemon[i]] = manhattan(points[i], (0, 0))

mini = float('inf')

while q:
    cur_dist, node, caught = heappop(q)

    if cur_dist > dist[node][caught]:
        continue

    if caught == (1 << n_pokemon) - 1:
        mini = min(cur_dist + manhattan(points[node], (0,0)), mini)

    for i, poke in enumerate(pokemon):
        if (1 << poke) & caught:
            continue

        alt = cur_dist + manhattan(points[i], points[node])
        if alt < dist[i][(1 <<  poke | caught)]:
            heappush(q, (alt, i, 1 <<  poke | caught))
            dist[i][(1 << poke) | caught] = alt
        

print(mini)
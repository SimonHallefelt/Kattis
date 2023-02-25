while(true):
    n = int(input())
    if n == 0:
        print("none")
        break

    primes = map(int, input().split())
    primes.append(1)
    x, y = map(int, input().split())
    


n = int(input())

words = []
for i in range(0, n):
    w = input()
    words.append(w)

targets = ["keys", "phone", "wallet"]
b = True
for t in targets:
    if t not in words:
        b = False
        print(t)

if b:
    print("ready")
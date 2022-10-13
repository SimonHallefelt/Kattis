from operator import contains


n = int(input())
s = input().split()
m = int(input())

set0 = set(s)
set1 = []
set2 = []
set3 = set()

for i in range(m):
    a,b,c = input().split()
    if a in set0:
        set2.append([a,b])
        if set1 == []:
            if c == "correct":
                set1.append([a,1,0])
            else:
                set1.append([a,0,1])
            set3.add(a)
        elif a in set3:
            for j in range(len(set1)):
                if set1[j][0] == a:
                    if c == "correct":
                        set1[j][1] += 1
                    else:
                        set1[j][2] += 1
                    break
        else:
            if c == "correct":
                set1.append([a,1,0])
            else:
                set1.append([a,0,1])
            set3.add(a)

correct = 0
incorrect = 0
temp = True
#t = True
f = False

for i in s:
    for j in range(len(set3)):
        if i == set1[j][0]:
            if set1[j][2] != 0 and incorrect == 0:
                if correct != 0:
                    incorrect = set1[j][2] * correct
                else:
                    incorrect = set1[j][2]
                f = True
            elif f:
                incorrect = incorrect * (set1[j][2] + set1[j][1]) + set1[j][2] * correct
            if temp and set1[j][1] != 0 and correct == 0:
                correct = 1
                temp = False
            else:
                temp = False
            correct *= set1[j][1]
            break

if incorrect + correct != 1:
    print(str(correct) +" correct")
    if f:
        print(str(incorrect) + " incorrect")
    else:
        print("0 incorrect")
else:
    s2 = ""
    for i in s:
        for j in range(len(set2)):
            if i == set2[j][0]:
                s2 += set2[j][1] + " "
                break

    print(s2)
    if correct == 1:
        print("correct")
    else:
        print("incorrect")

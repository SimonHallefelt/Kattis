a = input()
b = input()
c = input()

list1 = {'C': 0, 'C#': 1, 'D': 2, 'D#': 3, 'E': 4, 'F': 5, 'F#': 6, 'G': 7, 'G#': 8, 'A': 9, 'A#': 10, 'B': 11}
key_list = list(list1.keys())

list2 = []
list2.append(list1.get(a))
list2.append(list1.get(b))
list2.append(list1.get(c))
list2.sort()

if (list2[0]+4)%len(list1) == list2[1] and (list2[0]+7)%len(list1) == list2[2]:
    print(str(key_list[list2[0]]) + " major")
elif (list2[1]+4)%len(list1) == list2[2] and (list2[1]+7)%len(list1) == list2[0]:
    print(str(key_list[list2[1]]) + " major")
elif (list2[2]+4)%len(list1) == list2[0] and (list2[2]+7)%len(list1) == list2[1]:
    print(str(key_list[list2[2]]) + " major")
else:
    print("not a chord")

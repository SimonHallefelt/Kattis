import sys
sys.setrecursionlimit(10**5)


a,b,c,d = input().split()

aw = False
sett = set()
sett.add((a,b))
def run(aa,bb,i):
    global aw
    if not aw:
        if len(aa)*len(bb) and int(aa)*int(d) == int(bb)*int(c) and a != aa:
            print("possible")
            print(aa, bb)
            aw = True
        else:
            for j in range(i,len(aa)):
                for k in range(0,len(bb)):
                    if aa[j] == bb[k]:
                        aTemp = aa[:j] + aa[j+1:]
                        bTemp = bb[:k] + bb[k+1:]
                        if (aTemp,bTemp) not in sett:
                            sett.add((aTemp,bTemp))
                            run(aTemp,bTemp,j)
                            #print(aTemp,bTemp,j)


run(a,b,0)
if not aw:
    print("impossible")
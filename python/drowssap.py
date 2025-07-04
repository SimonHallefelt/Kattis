def rotate(msg, amount):
    return ((msg >> amount) | (msg << (20 - amount))) & 0xfffff

def transform(msg):
    msg = msg & 0xffffffffff
    
    a1 = msg & 0xfffff
    a2 = rotate(a1, 6)
    a3 = a2 ^ 0xc21f3
    a4 = a3 | 0xaaaaa

    b1 = (msg >> 20) & 0xfffff
    b2 = b1 ^ a4
    b3 = b2 | 0xaaaaa
    b4 = b3 ^ 0xbf83f
    b5 = rotate(b4, 6)

    a5 = a4 ^ b5

    print("a1:", a1, "a2:", a2, "a3:", a3, "a4:", a4, "a5:", a5)
    print("b1:", b1, "b2:", b2, "b3:", b3, "b4:", b4, "b5:", b5)

    return (a5 << 20) | b5



def revers_rotate(n, amount):
    return ((n >> (20 - amount)) | (n << amount)) & 0xfffff

def reverse(n):
    b5 = n & 0xfffff
    a5 = (n >> 20) & 0xfffff
    a4 = a5 ^ b5
    b4 = revers_rotate(b5, 6)
    b3 = b4 ^ 0xbf83f
    b1 = b3 ^ a4

    a2 = a4 ^ 0xc21f3
    a1 = revers_rotate(a2, 6)
    msg = (b1 << 20) | a1

    print("a4:", a4, "a5:", a5)
    print("b3:", b3, "b4:", b4, "b5:", b5)

    return msg

i = int(input())
msg = reverse(i)
print(msg)
a = msg & 0x00000fffff
b = msg & 0xfffff00000

print(a, 587286992506 & 0x00000fffff)
print(b, 587286992506 & 0xfffff00000)

print("transform(msg)", transform(msg))
print(transform(587286992506))

print("----------")
print("0 << 20 ", 0 << 20)
print("10 << 20 ", 10 << 20)
print("2 >> 8 ", 2 >> 8)
print("8 >> 2 ", 8 >> 2)
print("(10 << 20) >> 20 ", (10 << 20) >> 20)

print("----------")
print("6 | 4 ", 6 | 4)
print("6 | 2 ", 6 | 2)
print("6 | 1 ", 6 | 1)
print("6 | 8 ", 6 | 8)

print("----------")
print("6 ^ 4 ", 6 ^ 4)
print("6 ^ 2 ", 6 ^ 2)
print("6 ^ 1 ", 6 ^ 1)
print("6 ^ 8 ", 6 ^ 8)



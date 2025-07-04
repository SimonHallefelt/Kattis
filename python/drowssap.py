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

    return msg

i = int(input())
msg = reverse(i)
print(msg)

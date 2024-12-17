


expected = [2, 4, 1, 1, 7, 5, 4, 6, 0, 3, 1, 4, 5, 5, 3, 0]

def is_valid(out, expected_rev):
    print(str(out) + " == " + str(expected_rev[len(expected_rev)-len(out):]))

    return out[:] == expected_rev[len(expected_rev)-len(out):]

def compute(a):
    A = a
    B = 0
    C = 0

    out = []
    while True:
        B = A % 8; # 2,4, bst &A
        B = B^1 # 1, 1, bxl B 1

        C = (A // (2 ** B) & 0xffffffff); # 7,5, cdv A 2** B
        B = B ^ C # 4,6, bxc 
        A = A // 8 # 0,3, adv 3
        B = B ^ 4 # 1,4
        out.append(B % 8)

        if A == 0:
            break

    return out;

i=0
while i<1000000000000000:
    out = compute(i)
    print(str(i) + ": " + str(out))

    if is_valid(out, expected):
        print("VALID")
        i = i*8
    else:
        i+=1
        

# IF a = 0 Does nothing


def num_alpha(num):
    if num <= 26:
        return chr(64 + num)
    elif num%26 == 0:
        return num_alpha(num//26-1) + chr(90)
    else:
        return num_alpha(num//26) + chr(64 + num%26)
n = int(input())
print(num_alpha(n).lower())

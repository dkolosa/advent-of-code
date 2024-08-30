

ip_blcok = [(5,8), (0,2), (4,7)]

max = 9

open = []

for i in range(0, max+1):

    block = False

    for ip_min, ip_max in ip_blcok:
        if i >= ip_min and i <= ip_max and not block:
            block = True
            break

    if not block:
        open.append(i)

print([n for n in open])
print(min(open))
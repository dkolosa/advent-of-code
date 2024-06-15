



input_min = 138307
input_max = 654504

num_pass = 0

for num in range(input_min, input_max+1):

    num_str = str(num)
    i, j = 0,1
    pair = False
    no_order = False
    while j <= len(num_str)-1:

        n_i = num_str[i]
        n_j = num_str[j]

        if n_i > n_j:
            no_order = True
            break
        elif n_i == n_j:
            pair = True
        
        i += 1
        j += 1

    if pair and not no_order:
        num_pass += 1

print(num_pass)

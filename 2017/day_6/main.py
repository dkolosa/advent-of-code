
blocks = [0,2,7,0]
# blocks = [5,1,10,0,1,7,13,14,3,12,8,10,7,12,0,6]
num_blocks = len(blocks)

i = 1
seq = set(''.join([str(block) for block in blocks]))

while True:
    max_value = max(blocks)
    idx_max = blocks.index(max_value)
    blocks[idx_max] = 0
    idx = idx_max + 1

    while max_value > 0:
        if idx > num_blocks-1:
            idx = 0
        
        blocks[idx] += 1
        idx += 1
        max_value -= 1

    alloc = ''.join([str(block) for block in blocks])
    print(alloc)
    if alloc not in seq:
        i += 1
        seq.add(alloc)
    else:
        break
print(i)

from collections import Counter

width = 25
height = 6

test_input = "123456789012"

def read_file(filename):
    
    with open(filename, 'r') as f:
        line = f.readline()
    return line
breakpoint()
input = read_file('input.txt')

layer_size = width * height

min_zero = 1e6
result = 0

for layer in range(0,len(input),layer_size):

    pixel_count = Counter(input[layer:layer+layer_size])

    if min_zero > pixel_count.get('0',0):
        result = pixel_count.get('1',0) * pixel_count.get('2',0)
        min_zero = pixel_count.get('0')

print(result)

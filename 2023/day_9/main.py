input = '0 3 6 9 12 15'

input_int = map(int, input.split())


# iterate through the input list
# take the element of each element and the element before it
# Do this until all of the differences are 0

init_length = len(input_int)
idx = 1

while True:
    current = input_int[idx]

    diff = current - prev

    prev = current
    idx +=1

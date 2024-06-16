
def read_file(file):
    with open(file, 'r') as f:
        data = f.readlines()
    return data


def parse_digits(input):
    numbers = []
    for char in input:
        if char.isdigit():
            numbers.append(char)
    return int(numbers[0] + numbers[-1])

def parse_numbers(input: str):
    numbers = {"one" : "1", "two" : "2", "three": "3", "four": "4", "five": "5", "six": "6", "seven": "7", "eight": "8","nine" : "9"}
    for number_key, number_value in numbers.items():
        if number_key in input:
            input = input.replace(number_key, number_value)
    print(input)
    
    return input



input = read_file("test_input_2.txt")
numbers = []

for line in input:
    line = parse_numbers(line)
    # parsed = parse_digits(line)
    # print(parsed)
    # numbers.append(parsed)

# output = sum(numbers)
# print(output)


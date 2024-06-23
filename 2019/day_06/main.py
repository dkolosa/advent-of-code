from collections import defaultdict
from typing import List


input = '''COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L'''

def read_file(filename):
    
    with open(filename, 'r') as f:
        lines = f.read().splitlines()
    return lines

def set_orbits(input_lines: List[str]):
    orbit = defaultdict()
    for line in input_lines:
        parent, child = line.split(')')
        orbit[child] = parent

    return orbit

def count_orbit(child, orbits):
    count = 0
    while child != 'COM':
        parent = orbits[child]
        count += 1
        child = parent

    return count

orbits = set_orbits(input.split('\n'))

assert (count_orbit('D', orbits) == 3)
assert (count_orbit('L',orbits) == 7)
assert (count_orbit('COM', orbits) == 0)
assert(sum([count_orbit(child, orbits) for child in orbits.keys()]) == 42)

orbits = set_orbits(read_file('input.txt'))
print(count = sum([count_orbit(child, orbits) for child in orbits.keys()]))
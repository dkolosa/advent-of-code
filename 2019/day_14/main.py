"""
AOC 2019 Day 14:
https://adventofcode.com/2019/day/14
"""
import re

input = """10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"""


# split on => then split on ,
# place in dictionary  {"result" : "indr."}

input_lines = input.splitlines()


recipe = {}
for values in input_lines:
    vals_keys = re.split(r'=>', values)
    recipe[vals_keys[1]] = re.split(r',', vals_keys[0])


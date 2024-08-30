"""
day 12: The N-oody Problem
https://adventofcode.com/2019/day/12
"""
from copy import deepcopy

pos = [[-7, -1, 6], [6, -9, -9], [-12, 2, -7], [4, -17, -12]]
vel = [[0, 0, 0], [0, 0, 0], [0, 0, 0],[0,0,0]]
# pos = [[-8, -10, 0],
#     [5, 5, 10],
#     [2,-7, 3],
#     [9, -8, -3]]

pos_0 = deepcopy(pos)
vel_0 = deepcopy(vel)

def calc_velocity():
    for col in range(len(pos[0])):
        for row in range(len(pos)):
            value = pos[row][col]
            for other_row in range(len(pos)):
                if row != other_row:
                    if pos[other_row][col] > value:
                        vel[row][col] += 1
                    elif pos[other_row][col] < value:
                        vel[row][col] -= 1

def calc_energy(arr):
    energy = []
    for _, row in enumerate(arr):
        energy.append(sum([abs(element) for element in row]))
    return energy

def apply_velocity():
    for col in range(len(pos[0])):
        for row in range(len(pos)):
            pos[row][col] += vel[row][col]

def isPrevious():
    if pos == pos_0 and vel == vel_0:
        return True
    else:
        return False

def part_1():
    time = 10000
    for t in range(1, time + 1):
        # apply gravity
        calc_velocity()
        apply_velocity()

    
    pe = calc_energy(pos)
    ke = calc_energy(vel)
    print(sum([p*k for p,k in zip(pe,ke)]))

def part_2():

    t = 0
    while True:
        # apply gravity
        calc_velocity()
        apply_velocity()

        if isPrevious():
            print(t)
            break
        t += 1

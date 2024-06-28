
mass = 100756

def read_file(name):

    with open(name, 'r') as f:
        lines = f.read().splitlines()

    return lines

def calc_fuel(mass):
    fuel = mass // 3 - 2
    return fuel

if __name__ == '__main__':

    masses = read_file('day_01_input.txt')
    # masses = [14,1969]
    fuel_sum = 0
    for mass in masses:
        inter_sum = 0
        fuel = calc_fuel(int(mass))
        inter_sum = fuel
        while fuel >= 0:
            fuel = calc_fuel(int(fuel))
            if fuel <= 0:
                break
            inter_sum += fuel

        fuel_sum += inter_sum
    
    print(fuel_sum)
    


oppcodes = (99,1,2)
window = 4

def read_file(filename):

    with open(filename, 'r') as f:
        data = f.readline().split(',')
        int_data = [int(n) for n in data]
    return int_data



def main():

    input_file = 'input_1.txt'
    #read the input file
    # test_input = [1,9,10,3,2,3,11,0,99,30,40,50]

    input_seq =  read_file(input_file)
    input_seq[1] = 12
    input_seq[2] = 2

    #Iterate over the input by steps of window
    for i in range(0,len(input_seq), window):
        seq = input_seq[i:i+window]
        op = i
    # check opcode
        if seq[0] == oppcodes[0]:
            print('Answer:',input_seq[0])
        elif seq[0] == oppcodes[1]:
            input_seq[seq[3]] = input_seq[seq[1]] + input_seq[seq[2]]
        elif seq[0] == oppcodes[2]:
            input_seq[seq[3]] = input_seq[seq[1]] * input_seq[seq[2]]




if __name__ == '__main__':
    main()
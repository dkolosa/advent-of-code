oppcodes = (99,1,2, 3, 4)
window = 4

def read_file(filename):

    with open(filename, 'r') as f:
        data = f.readline().split(',')
        int_data = [int(n) for n in data]
    return int_data



def main():

    # input_file = 'input_1.txt'
    #read the input file
    input_seq = [1,9,10,3,2,3,11,0,99,30,40,50]

    input_seq = [1002,4,3,4,33]

    # input_seq =  read_file(input_file)
    i = 0
    #Iterate over the input by steps of window
    while i <= len(input_seq):
        breakpoint()
        op_seq = str(input_seq[i])
        op = int(op_seq[-2:])
        param_1 = int(op_seq[-3]) if len(op_seq) >= 3 else 0
        param_2 = int(op_seq[-4]) if len(op_seq) >= 4 else 0
        param_3 = int(op_seq[-5]) if len(op_seq) == 5 else 0

    # check opcode
        if input_seq[i] == oppcodes[0]:
            print('Answer:',input_seq[i])
            exit(1)
        elif input_seq[i] == oppcodes[1]:
            input_seq[input_seq[i+3]] = input_seq[i+1] + input_seq[i+2]
            i+=4
        elif input_seq[i] == oppcodes[2]:
            input_seq[i+3] = input_seq[i+1] * input_seq[i+2]
            i+=4
        elif input_seq[i] == oppcodes[3]:
            param = int(input())
            input_seq[input_seq[i+1]] = param
            i+=2
        elif input_seq[i] == oppcodes[4]:
           print(input_seq[i+1])
           i+=2
        else:
            raise "Invalid opcode"





if __name__ == '__main__':
    main()
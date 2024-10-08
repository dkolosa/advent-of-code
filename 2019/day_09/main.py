from sys import exit
from enum import Enum


class Opcode(Enum):
    EXIT = 99
    ADD  = 1
    MULTIPLY = 2
    INPUT = 3
    PRINT = 4
    JUMP_TRUE = 5
    JUMP_FALSE = 6
    LESS_THAN = 7 
    EQUALS = 8
    RELATIVE_BASE = 9

def read_file(filename):

    with open(filename, 'r') as f:
        data = f.readline().split(',')
        int_data = [int(n) for n in data]
    return int_data

def get_seq(input_seq, i, param_1, param_2):
    if param_1 == 1:
        seq_1 = input_seq[i+1]
    elif param_1 == 0:
        seq_1 = input_seq[input_seq[i+1]]
    elif param_1 == 2:
        seq_1 = input_seq[i+input_seq[i+1]]            
    if param_2 == 1:
        seq_2 = input_seq[i+2]
    elif param_2 == 0:
        seq_2 = input_seq[input_seq[i+2]]
    elif param_2 == 2:
        seq_1 = input_seq[i+input_seq[i+2]]            
    return seq_1, seq_2


def main():

    # input_file = 'input_1.txt'
    #read the input file
    # input_seq = [1,9,10,3,2,3,11,0,99,30,40,50]
    # input_seq = [1002,4,3,4,33,3,1]
    # input_seq = [3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]
    # input_seq = [3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]
    # input_seq = [3,3,1105,-1,9,1101,0,0,12,4,12,99,1]
    input_seq = [3,225,1,225,6,6,1100,1,238,225,104,0,1002,114,46,224,1001,224,-736,224,4,224,1002,223,8,223,1001,224,3,224,1,223,224,223,1,166,195,224,1001,224,-137,224,4,224,102,8,223,223,101,5,224,224,1,223,224,223,1001,169,83,224,1001,224,-90,224,4,224,102,8,223,223,1001,224,2,224,1,224,223,223,101,44,117,224,101,-131,224,224,4,224,1002,223,8,223,101,5,224,224,1,224,223,223,1101,80,17,225,1101,56,51,225,1101,78,89,225,1102,48,16,225,1101,87,78,225,1102,34,33,224,101,-1122,224,224,4,224,1002,223,8,223,101,7,224,224,1,223,224,223,1101,66,53,224,101,-119,224,224,4,224,102,8,223,223,1001,224,5,224,1,223,224,223,1102,51,49,225,1101,7,15,225,2,110,106,224,1001,224,-4539,224,4,224,102,8,223,223,101,3,224,224,1,223,224,223,1102,88,78,225,102,78,101,224,101,-6240,224,224,4,224,1002,223,8,223,101,5,224,224,1,224,223,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1107,226,677,224,102,2,223,223,1006,224,329,101,1,223,223,1108,226,677,224,1002,223,2,223,1005,224,344,101,1,223,223,8,226,677,224,102,2,223,223,1006,224,359,1001,223,1,223,1007,226,677,224,1002,223,2,223,1005,224,374,101,1,223,223,1008,677,677,224,1002,223,2,223,1005,224,389,1001,223,1,223,1108,677,226,224,1002,223,2,223,1006,224,404,1001,223,1,223,1007,226,226,224,1002,223,2,223,1005,224,419,1001,223,1,223,1107,677,226,224,1002,223,2,223,1006,224,434,101,1,223,223,108,677,677,224,1002,223,2,223,1005,224,449,1001,223,1,223,1107,677,677,224,102,2,223,223,1005,224,464,1001,223,1,223,108,226,226,224,1002,223,2,223,1006,224,479,1001,223,1,223,1008,226,226,224,102,2,223,223,1005,224,494,101,1,223,223,108,677,226,224,102,2,223,223,1005,224,509,1001,223,1,223,8,677,226,224,1002,223,2,223,1006,224,524,101,1,223,223,7,226,677,224,1002,223,2,223,1006,224,539,101,1,223,223,7,677,226,224,102,2,223,223,1006,224,554,1001,223,1,223,7,226,226,224,1002,223,2,223,1006,224,569,101,1,223,223,107,677,677,224,102,2,223,223,1006,224,584,101,1,223,223,1108,677,677,224,102,2,223,223,1006,224,599,1001,223,1,223,1008,677,226,224,1002,223,2,223,1005,224,614,1001,223,1,223,8,677,677,224,1002,223,2,223,1006,224,629,1001,223,1,223,107,226,677,224,1002,223,2,223,1006,224,644,101,1,223,223,1007,677,677,224,102,2,223,223,1006,224,659,101,1,223,223,107,226,226,224,1002,223,2,223,1006,224,674,1001,223,1,223,4,223,99,226]

    i = 0
    #Iterate over the input by steps of window
    while i <= len(input_seq):
        op_seq = str(input_seq[i])
        if len(op_seq) >= 2:
            op = int(op_seq[-2:])
            param_1 = int(op_seq[-3]) if len(op_seq) >= 3 else 0
            param_2 = int(op_seq[-4]) if len(op_seq) >= 4 else 0
            param_3 = int(op_seq[-5]) if len(op_seq) == 5 else 0
        else:
            op = int(op_seq)
            param_1 = 0
            param_2 = 0
            param_3 = 0
    # check opcode
        if op == Opcode.EXIT.value:
            print('exiting')
            exit(0)
        elif op == Opcode.ADD.value:
            seq_1, seq_2 = get_seq(input_seq, i, param_1, param_2)
            if param_3 == 0: 
                input_seq[input_seq[i+3]] =  seq_1 + seq_2 
            elif param_3 == 1:
                input_seq[i+3] = seq_1 + seq_2
            i+=4
        elif op == Opcode.MULTIPLY.value:
            seq_1, seq_2 = get_seq(input_seq, i, param_1, param_2)
            if param_3 == 0: 
                input_seq[input_seq[i+3]] =  seq_1 * seq_2 
            elif param_3 == 1:
                input_seq[i+3] = seq_1 * seq_2
            i+=4
        elif op == Opcode.INPUT.value:
            param = int(input('input:'))
            input_seq[input_seq[i+1]] = param
            i+=2
        elif op == Opcode.PRINT.value:
            if param_1 == 0:
               val = input_seq[input_seq[i+1]]
            else:
                val = input_seq[i+1]
            print('PRINT: ',val)
            i+=2
        elif op == Opcode.JUMP_TRUE.value:
            seq_1,seq_2 = get_seq(input_seq, i ,param_1, param_2)
            if seq_1 != 0:
                i = seq_2
            else:
                i += 3
        elif op == Opcode.JUMP_FALSE.value:
            seq_1,seq_2 = get_seq(input_seq, i ,param_1, param_2)
            if seq_1 == 0:
                i = seq_2
            else:
                i += 3
        elif op == Opcode.LESS_THAN.value:
            seq_1, seq_2 = get_seq(input_seq, i ,param_1, param_2)
            if seq_1 < seq_2:
                if param_3 == 0:
                    input_seq[input_seq[i+3]] = 1 
                elif param_3 == 1:
                    input_seq[i+3] = 1 
            else:
                if param_3 == 0:
                    input_seq[input_seq[i+3]] = 0 
                elif param_3 == 1:
                    input_seq[i+3] = 0 
            i +=4
        elif op == Opcode.EQUALS.value:
            seq_1, seq_2 = get_seq(input_seq, i ,param_1, param_2)
            if seq_1 == seq_2:
                if param_3 == 0:
                    input_seq[input_seq[i+3]] = 1 
                elif param_3 == 1:
                    input_seq[i+3] = 1 
            else:
                if param_3 == 0:
                    input_seq[input_seq[i+3]] = 0 
                elif param_3 == 1:
                    input_seq[i+3] = 0 
            i+=4
        else:
            print(f"Invalid Opcode {input_seq[i]}")





if __name__ == '__main__':
    main()
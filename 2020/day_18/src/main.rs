use std::fs;
// Day 18

fn main() {

    let filename = "input.txt";

    let contents: Vec<String> = fs::read_to_string(filename).expect("file not found")
                            .lines()
                            .map(|s| s.to_string())
                            .collect();

    
    for line in contents {

        // You will have two stacks (number stack and operator stack)
        // one result variable
        // Push on the stack for ( and pop on the stack for ) until hit a matching (
        // 
        // let mut result = 0;
        let mut op: char = '_';

        let mut prev_value = 0;

        let mut idx = 0;
        let char_line: Vec<char> = line.chars().collect();

        while idx < char_line.len() {
            let cn = char_line[idx];

            if cn.is_digit(10) {
                math_op(cn, op, &mut prev_value);
            } else if cn == '+' || cn =='*' {
                op = cn;
            } else if cn == '(' { }
            idx += 1;
        }
        println!("{}", prev_value)

    }
    
    
}

fn math_op(cn: char, op: char, prev_value: &mut i32) {
    let dig = cn.to_digit(10).unwrap() as i32;
    match op {
        '+' => *prev_value += dig,
        '*' => *prev_value *= dig,
        _ => *prev_value = dig 
    }
}


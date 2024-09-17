use std::fs;
// Day 18

fn main() {

    let filename = "input_p1.txt";

    let contents: Vec<String> = fs::read_to_string(filename).expect("file not found")
                            .lines()
                            .map(|s| s.to_string())
                            .collect();

    let mut final_sum:i64 = 0;
    
    for line in contents {

        // let mut result = 0;
        let mut op: char = '_';
        let mut prev_value:i64 = 0;

        let mut idx = 0;

        let char_line: Vec<char> = line.chars().collect();
        while idx < char_line.len() {
            parse_chars(&char_line, &mut idx, &mut op, &mut prev_value);
        }
        final_sum += prev_value;
        println!("{}", prev_value);
    }
    println!("Final Sum: {}", final_sum);
}

fn parse_chars(char_line: &Vec<char>, idx: &mut usize, op: &mut char, prev_value: &mut i64) {
    while *idx < char_line.len(){
        let cn = char_line[*idx];
        *idx += 1;
        if cn == ')' {
            return;
        }
        if cn.is_digit(10) {
            let dig = cn.to_digit(10).unwrap() as i64;
            math_op(dig, *op, prev_value);
        } else if cn == '+' || cn =='*' {
            *op = cn;
        } else if cn == '(' { 
            let mut inner_value = 0;
            parse_chars(char_line, idx,&mut '_', &mut inner_value);
            math_op(inner_value, *op, prev_value);
        }
    }
}

fn math_op(dig:i64, op: char, prev_value: &mut i64) {
    match op {
        '+' => *prev_value += dig,
        '*' => *prev_value *= dig,
        _ => *prev_value = dig 
    }
}


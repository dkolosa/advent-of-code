
use std::{fs, str::FromStr};

fn part1(mut int_values:Vec<i32>, target:i32) {

    let mut r: usize = 0;
    let mut l: usize = 1;
    let in_length: usize = int_values.len();

    loop{

        if (int_values[r] + int_values[l]) == target {
            let answer = int_values[r] * int_values[l];
            println!("Answer: {}", answer);
            break;   
        }
        l = l+1;

        if l == in_length{
            r = r+1;
            l = r+1;
        }

    }
}

fn main() {

    let file_path = "input_p1.txt";
    let target = 2020;
    let input_txt = fs::read_to_string(file_path).expect("Cannot read file");

    let values: Vec<&str> = input_txt.split("\n").collect();
    let mut int_values: Vec<i32> = Vec::new();

    for str_vales in values.iter().enumerate(){
        int_values.push(i32::from_str(str_vales.1).unwrap());
    }

    part1(int_values, target);
  
}
use std::{collections::HashSet, fs};

fn main() {

    let file_name = "input_p1.txt";
    let contents: String = fs::read_to_string(file_name)
                            .expect("file not found");
    let lines:Vec<&str> = contents.lines().collect();

    let mut visited: HashSet<i32> = HashSet::new();

    let mut i:i32 = 0;
    let mut accu:i32 = 0;

    while !visited.contains(&i)  {
        
        let line: Vec<&str> = lines[i as usize].split_whitespace().collect();

        let cmd = line[0];
        let arg = line[1].to_string();
        let arg_int = arg.parse::<i32>().unwrap();
        visited.insert(i);
        match cmd {
            "nop" => {i += 1},
            "acc" => {i += 1; accu += arg_int},
            "jmp" =>  i += arg_int,
            &_ => println!("")            
        }
    }
    println!("Accumulator: {}", accu);

}

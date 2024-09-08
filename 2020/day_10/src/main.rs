use std::fs;

fn main() {

    let file_name = "input_p1.txt";
    let contents = fs::read_to_string(file_name)
                            .expect("File not found");
    let mut lines: Vec<i32>= contents.lines()
                            .map(|s| s.parse::<i32>().unwrap())
                            .collect();
    lines.push(0);
    lines.push(lines.iter().max().unwrap() + 3 as i32);

    lines.sort();

    let mut diff_one : i32 = 0;
    let mut diff_three: i32 = 0;

    
    for num in 0..lines.len()-1{
        let diff = lines[num+1] - lines[num];
        match  diff {
            3 => diff_three += 1,
            1 => diff_one += 1,
            _ => ()
        }

    }

    println!("Ones:{}, Threes:{}", diff_one, diff_three);
    println!("Answer: {}", diff_one * diff_three);

}

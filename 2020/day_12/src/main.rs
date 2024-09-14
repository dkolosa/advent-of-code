use std::{fs};

fn get_direction(direction_mag:&str) -> (char, i32){

    let dir: char = direction_mag.chars().nth(0).unwrap();
    let magnitude = direction_mag[1..].parse::<i32>().unwrap();

    return (dir, magnitude)
}

fn do_rotation(angle:i32) -> i32 {

    let norm_angle = angle % 360;

    if angle < 0{
        360 + norm_angle
    } else {
        norm_angle   
    }

}

fn main() {

    let filename = "input_p1.txt";

    let contents: Vec<String> = fs::read_to_string(filename)
                                            .expect("File not found")
                                            .lines()
                                            .map(|s| s.to_string())
                                            .collect();

    let mut current_position = (0,0);

    let mut current_direction = 0;
    
    for direction in contents {

        let dir_steps = get_direction(&direction);

        match dir_steps.0 {
            'N' => current_position.1 += dir_steps.1,
            'S' => current_position.1 -= dir_steps.1,
            'E' => current_position.0 += dir_steps.1,
            'W' => current_position.0 -= dir_steps.1,
            'L' => current_direction = do_rotation(dir_steps.1 + current_direction),
            'R' => current_direction = do_rotation(current_direction - dir_steps.1),
            'F' => match current_direction {
                    0 => current_position.0 += dir_steps.1,
                    90 => current_position.1 += dir_steps.1,
                    180 => current_position.0 -= dir_steps.1,
                    270 => current_position.1 -= dir_steps.1,
                    _ => println!("Invalid Angle")
                }
            _ => println!("Incorrect Value")
        };
    }
    println!("{}", current_position.0.abs() + current_position.1.abs());

}

use std::fs;

fn main() {

    let file_name = "input_p1.txt";

    let contents = fs::read_to_string(file_name)
                                            .expect("Error reading file");
    let char_array: Vec<Vec<char>> = contents.lines()
                                    .map(|line| line.chars().collect()).collect();

    let move_right = 3;
    let move_down = 1;

    let height = char_array.len();
    let width = char_array[0].len();

    let mut num_trees = 0;

    let mut start_x =0;
    let mut start_y = 0;
    for _ in 0..height {

        if char_array[start_y][start_x] == '#'{
            num_trees += 1;
        }

        start_x += move_right;
        start_y += move_down;        
        // check if you are at the end of array (width-wise)
        if start_x >= width {
            start_x = start_x % width; 
        }
    }

    println!("Number of trees: {}", num_trees);

}

use std::{char, fs};

fn print_seats(char_arr: &mut Vec<Vec<char>>){
    
    for row_idx in 0..char_arr.len(){
        for col_idx in 0..char_arr[0].len(){
           print!("{}",char_arr[row_idx][col_idx]);
        }
        println!("");
    }

        println!("");
}

fn poppulate_seats(char_arr: &mut Vec<Vec<char>>) {

    let max_row = char_arr.len();
    let max_col = char_arr[0].len();
    
    for row_idx in 0..max_row{
        for col_idx in 0..max_col{
            match char_arr[row_idx][col_idx] {
                'L' => char_arr[row_idx][col_idx] = '#',
                _ => {}
            }
            
        }
    }
}

fn count_seats(char_arr: &mut Vec<Vec<char>>) -> i64{

    let max_row = char_arr.len();
    let max_col = char_arr[0].len();
    let mut num_seats = 0;

    for row_idx in 0..max_row{
        for col_idx in 0..max_col{
            match char_arr[row_idx][col_idx] {
                '#' => num_seats += 1,
                _ => {}
            }
            
        }
    }
    return num_seats;

}

fn empty_seats(char_arr: &mut Vec<Vec<char>>, empty_seat: Vec<(usize,usize)>, remove_seat: bool) {

    let assignment = if remove_seat { 'L' } else {'#'};
    
    for seat in empty_seat {
        char_arr[seat.0][seat.1] = assignment;
    }    
}

fn check_seats(char_arr: &mut Vec<Vec<char>>, remove_seat: bool) {
    let max_row = char_arr.len();
    let max_col = char_arr[0].len();
    let max_adj_seats = 4;
    let mut change_seat: Vec<(usize, usize)> = Vec::new();

    let direction: [(i32, i32); 8] = [(1, 0), (-1, 0), (0,1), (0,-1),
                                        (1,-1), (1,1), (-1,-1), (-1,1) ];
    for row_idx in 0..max_row{
        for col_idx in 0..max_col{
            let mut accum = 0;
                if char_arr[row_idx][col_idx] != '.' {
                    for (dr, dc) in direction.iter(){
                        let new_row = row_idx as i32 + dr;
                        let new_col = col_idx as i32 + dc;

                        if new_row >= 0 && new_row < max_row as i32 &&
                            new_col >= 0 && new_col < max_col as i32 &&
                            char_arr[new_row as usize][new_col as usize] == '#' {
                                accum +=1;
                        } 
                    }

                    if accum >= max_adj_seats && char_arr[row_idx][col_idx] == '#'{
                        change_seat.push((row_idx, col_idx));
                    } else if accum == 0 && char_arr[row_idx][col_idx] == 'L'{
                        change_seat.push((row_idx, col_idx));
                   }
                    
                }
            
        }
    }
    empty_seats(char_arr, change_seat, remove_seat);
    print_seats(char_arr);
}


fn main() {

    let filename = "input_p1.txt";
    let contents: Vec<String> = fs::read_to_string(filename)
                                            .expect("Error reading file")
                                            .lines()
                                            .map(|c| c.to_string()).collect();
    let mut char_arr: Vec<Vec<char>> = contents.iter()
                              .map(|c| c.chars().collect())
                              .collect();

    poppulate_seats(&mut char_arr);
    let mut prev_arr =char_arr.clone();
    let mut remove_swt = true;
    let mut count = 1;
    let seats = loop {
        // Make a copy of the previous array to check if if doesnt change
        // Be better to track changes instead fo comparing the entire ds
        remove_swt = count % 2 == 1;
        check_seats(&mut char_arr, remove_swt);
        
        if prev_arr == char_arr {
            break count_seats(&mut char_arr);
        }
        prev_arr.clear();
        prev_arr = char_arr.clone();
        count += 1

    };

    println!("Seats {}", seats);
        
}

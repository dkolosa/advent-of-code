use std::fs::read_to_string;

fn main() {

    let file_name = "input_p1.txt";
    let content = read_to_string(file_name).expect("file not found");
    let lines: Vec<&str> = content.lines().collect();
    let mut max_id = 0.0;

    
    for seats in lines {

    // let seats = "BBFFBBFRLL".to_string();

        let mut max_row:f32= 127.0;
        let mut max_col:f32 = 7.0;
        let mut min_row:f32 = 0.0;
        let mut min_col:f32 = 0.0;
        
        let mut final_row:f32 = 0.0;
        let mut final_col:f32 = 00.0;

        for (i, seat) in seats.chars().enumerate() {

            if i < 6 {
                let seat_row = (min_row + max_row) / 2.0;
                match seat {
                    'B' => min_row = seat_row.ceil(),
                    'F' => max_row = seat_row.floor(),
                    _ => println!("Unexpected Value!"),
                } 
            } else if i == 6 {
                match seat {
                    'B' => final_row = max_row,
                    'F' => final_row = min_row,
                    _ => println!("Invalid Value"),
                }
            } else if i == 9 {
                match seat {
                    'R' => final_col = max_col,
                    'L' => final_col = min_col,
                    _ => println!("Invalid Value"),
                }
            }else {
                let seat_col = (min_col + max_col) / 2.0;
                match seat {
                    'R' => min_col = seat_col.ceil(),
                    'L' => max_col = seat_col.floor(),
                    _ => println!("Unexprected Value!!"),
                }
            }
        }
        let seat_id = final_row * 8.0 + final_col;
        if seat_id > max_id{
            max_id = seat_id;
        }
        // println!("Seat ID: {}", final_row * 8.0 + final_col);
    }
    println!("Max ID: {}",max_id)
}

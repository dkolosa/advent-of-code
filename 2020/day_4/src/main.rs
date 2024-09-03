use std::{collections::HashSet, fs};

fn main() {

    let file_name = "input_p1.txt";
    let contents = fs::read_to_string(file_name)
                                            .expect("File not found");

    let entries: Vec<&str> = contents.split("\n\n").collect();


    let valid_fields = HashSet::from(["byr", "iyr", "eyr", "hgt", "hgt", "hcl", "ecl", "pid"]);
    let mut valid_passprots = 0;

    // Iterate through each of the entries to check if the field exists in the valid fields
    for entry in entries {
        // Split the entries into individual fields and only keep the keys
        let fields: Vec<&str> = entry.split_whitespace()
                      .filter_map(|x| x.split(':').next()).collect();
        
        let mut valid = true;
        for valid_field in valid_fields.clone() {
            if !fields.contains(&valid_field){
                valid = false;
                break;
            }
        }
        if !valid{
            continue;
        }
        valid_passprots+=1;


    }

    println!("Valid Passports {}", valid_passprots);
    println!("Done!");
}

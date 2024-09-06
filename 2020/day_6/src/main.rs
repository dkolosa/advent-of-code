use std::{collections::{HashMap, HashSet}, fs};

fn main() {

    let file_nmae = "input_p1.txt";
    let contents: Vec<String> = fs::read_to_string(file_nmae)
                                            .expect("Error Reading File")
                                            .split("\n\n")
                                            .map(|x| x.to_string())
                                            .collect();
    let entries: Vec<String> = contents.iter()
                                    .map(|s| s.replace("\n", ""))
                                    .collect();

    let mut entry_vec = Vec::new();
    for entry in &entries{
        let mut entries_counter: HashMap<char, usize> = HashMap::new();
        for c in entry.chars(){
            *entries_counter.entry(c).or_insert(0) += 1;
        }
        entry_vec.push(entries_counter);
    } 

    

    let sum_entries: i32 = entries.iter()
                    .map(|x| { 
                        let unq_chars: HashSet<char> = x.chars().collect();
                        unq_chars.len() as i32})
                    .sum();

    println!("Sum: {}", sum_entries);
    
}

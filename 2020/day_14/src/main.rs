use std::{collections::HashMap, io};
use regex::Regex;

struct B36(u64);

impl B36 {

    fn new(value :u64)-> Self {
        Self(value & ((1u64 << 36)- 1))
    }

    fn set_bit(&mut self, position: usize) {
        if position < 36 {
            self.0 |= 1 << position;
        }
    }

    fn check_bit_set(&self, position: usize ) -> bool   {
        if position < 36 {
            (self.0 >> position) & 1 == 1
        } else {
            false
        }
    }

    fn clear_bit(&mut self, position: usize) {
        if position < 36 {
            self.0 &= !(1 << position);
        }
    }
    
    fn value(&self) -> u64 {
        self.0
    }

    fn mask_to_or(mask:&String) -> u64{

        let mask_str:String = mask.chars()
                        .map(|c| {if c == '1' {'1'} else {'0'}})
                        .collect();
        let binary_mask = u64::from_str_radix(&mask_str, 2).expect("Invalid value");

        // Ensure it is 36-bits
        binary_mask & ((1u64 << 36)- 1)
                        
    }

    fn mask_to_and(mask:&String) -> u64 {

        let mask_str: String = mask.clone().chars()
                        .map(|c| {if c == '0' {'0'} else {'1'}})
                        .collect();

        let binary_mask = u64::from_str_radix(&mask_str, 2).expect("Invalid value");

        // Ensure it is 36-bits
        binary_mask & ((1u64 << 36)- 1)
    }


    fn apply_mask(&mut self, mask: &String) {
 
        let and_mask: u64 = Self::mask_to_and(mask);
        let or_mask: u64 = Self::mask_to_or(mask);

        self.0 = (self.0 & and_mask) | or_mask
    }


}

fn main() {


    let mut addr_space: HashMap<i64, B36> = HashMap::new();
    let file_path = "input.txt";
    extract_values(file_path);
    


    bit.apply_mask(&mask);
    let index = 7;
    addr_space.insert(index, bit);

    
    let mut sum = 0;
    for (_, value) in addr_space.iter(){
        sum += value.value();
    }


    println!("Total sum: {}", sum);

}

fn extract_values(file_path: &str) {

    let reg_mask =  Regex::new(r"mask = ([X01]+)").unwrap();
    let reg_mem = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    
    let mut mask= String::new();


    for line in reader.lines() {
        let line = line?;

        if let Some(cap) = reg_mask.captures(line){
            mask = cap[1].to_string();
    
        }

        if let Some(cap) = reg_mem.captures(line){
            let address = &cap[1];
            let value = &cap[2];
        }

    }

    for cap in mem_re.captures_iter(input_data) {
        let address = &cap[1];
        let value = &cap[2];
        println!("Address: {}, Value: {}", address, value);
    }
    }
use std::string;

struct B36(u64);
impl B36 {

    fn new(value :u64)-> Self {
        Self(value & ((1u64 << 36))- 1)
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
                        .map(|c| {if c == 'X' {'1'} else {'0'}})
                        .collect();
        let binary_mask = u64::from_str_radix(&mask_str, 2).expect("Invalid value");

        // Ensure it is 36-bits
        binary_mask & ((1u64 << 36))- 1
                        
    }

    fn mask_to_and(mask:&String) -> u64 {

        let mask_str: String = mask.clone().chars()
                        .map(|c| {if c == 'X' {'0'} else {'1'}})
                        .collect();
        let binary_mask = u64::from_str_radix(&mask_str, 2).expect("Invalid value");

        // Ensure it is 36-bits
        binary_mask & ((1u64 << 36))- 1
    }


    fn apply_mask(&self, mask: &String) -> u64{
 
        let and_mask: u64 = Self::mask_to_and(mask);

        let or_mask: u64 = Self::mask_to_or(mask);

        (self.0 & and_mask) | or_mask
    }



}

fn main() {


    let bit: B36 = B36::new(101);
    let mask:String = String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
    let masked_bit  = bit.apply_mask(&mask);

    let mut sum = 0;

    if B36::mask_to_and(&mask) ^ masked_bit == 0 {
        //Take the value
        sum += masked_bit;
    }

    println!("Total sum: {}", sum);

}
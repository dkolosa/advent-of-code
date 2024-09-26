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
    
    fn get_value(&self) -> u64 {
        self.0
    }

    fn apply_mask(&self, mask: u64) -> u64{
        
        let and_mask: u64 = 0b111;
        let or_mask: u64 = 0b111;

        (self.0 & and_mask) | or_mask

    }


}

fn main() {


    let mut bit: B36 = B36::new(0b1011);
    bit.set_bit(20);
    println!("The bit value {}", bit.get_value());

    bit.clear_bit(20);
    println!("Bit cleared {}", bit.get_value());
    
    println!("is bit 12 set {}", bit.check_bit_set(12));
}
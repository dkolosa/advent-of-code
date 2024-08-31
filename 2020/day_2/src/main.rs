use std::fs;


struct Password {
    pub min: i32,
    pub max: i32,
    pub key: char,
    pub code: String,
}

impl Password {
    fn count(&self) -> i32{
        self.code.chars().filter(|c| *c == self.key).count() as i32
    }

    fn contain_one(&self) -> bool {

        if self.code.chars().nth((self.min-1) as usize).expect("invalid") == self.key && 
            self.code.chars().nth((self.max-1) as usize).expect("invalid")  == self.key {
            return false;
        
        } else if self.code.chars().nth((self.min-1) as usize).expect("invalid") != self.key && 
        self.code.chars().nth((self.max-1) as usize).expect("invalid")  == self.key {
            return true;
        } else if self.code.chars().nth((self.min-1) as usize).expect("invalid") == self.key &&
         self.code.chars().nth((self.max-1) as usize).expect("invalid")  != self.key {
            return true;
        }
        else {
            return false;
        }
    }

    fn is_valid(&self) -> bool{
        let char_count: i32 = self.count();
        if self.min <= char_count && self.max >= char_count{
            true
        } else {
            false
        }
    }
}


fn main() {

    let fileName = "input_p1.txt";

    let contents = fs::read_to_string(fileName)
                                          .expect("Cannot read file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut total:i32 = 0;

    for line in lines {
        let line_contents: Vec<&str> = line.split_whitespace().collect();
        let mut range = line_contents[0].split('-');
        let letter = line_contents[1];
        let seq = line_contents[2].to_string();

        let password = Password{min: range.next().expect("invaid string").parse().expect("NAN"),
                                max:range.next().expect("invalid String").parse().expect("NAN"),
                                key: letter.chars().nth(0).expect("No char found"),
                                code: seq
                                };

        // if password.is_valid() {
        //     total += 1;
        // }

        if password.contain_one(){
            total += 1;
        }
    }
    println!("total {}", total);

    



}

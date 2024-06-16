use std::{fs, collections::HashMap };

fn read_file(filename: &str) -> Vec<String>{

    let mut data = Vec::new();
    // let mut bids: Vec<i32> = Vec::new();
    for line in  fs::read_to_string(filename).unwrap().lines(){
        data.push(line.to_string());
    }
    data
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Strength {
    FIVEOFKIND,
    FOUROFKIND,
    FULLHOUSE,
    THREEOFKIND,
    TWOPAIR,
    ONEPAIR,
    HIGHCARD
}  

struct Deck {
    hand: String,
    strength: Strength,
    bid: i32,
    rank: i32,
}

impl Deck {
    fn get_number_and_letter(&self) -> (HashMap<char, i32>, HashMap<char, i32>) {
        let mut letter_counts = HashMap::new();
        let mut number_counts = HashMap::new();
        
        let hand = &self.hand;

        for ch in hand.chars() {
            if ch.is_alphabetic() {
                *letter_counts.entry(ch.to_ascii_lowercase()).or_insert(0) += 1;
            } else if ch.is_numeric() {
                *number_counts.entry(ch).or_insert(0) += 1;
            }
        }
        (letter_counts, number_counts)
    }
    fn check_strength(&mut self) {
        // Determine the strengh of the hand
        // There are 5 cards, and 7 deck
        let letter_counts: HashMap<char, i32>;
        let number_counts: HashMap<char, i32>;

        (letter_counts, number_counts) = self.get_number_and_letter();
        let mut max_letter: i32 = 0;
        let mut max_number: i32 = 0;
        let mut snd_letter :bool = false;
        let mut snd_number: bool = false;

        max_letter = match letter_counts.values().max() {
            Some(max_value) => {*max_value},
            None => {0},
        };
        max_number = match number_counts.values().max() {
            Some(max_value) => {*max_value},
            None => {0},
        };

        snd_letter = letter_counts.values().any(|&val| val == 2);
        snd_number = number_counts.values().any(|&val| val == 2);

        if max_number == 5 || max_letter == 5{
            self.strength = Strength::FIVEOFKIND;
        } else if max_number == 4  || max_letter == 4{
            self.strength = Strength::FOUROFKIND;
        } else if (max_number == 3 && snd_letter) ||  (snd_number && max_letter == 3) || (max_number == 3 && snd_number) || (max_letter == 3 && snd_letter) {
            self.strength = Strength::FULLHOUSE;
        } else if max_number == 3  || max_letter == 3{
            self.strength = Strength::THREEOFKIND;
        } else if max_number == 2  || max_letter == 2 {
            self.strength = Strength::TWOPAIR;
        } else {
            self.strength = Strength::HIGHCARD;
        }
    }

    fn calculate_score(&self) -> i32 {
        return self.rank * self.bid;
    }

}


fn main() {
    let filename = "./src/min_input.txt";
    
    let mut lines: Vec<String> = Vec::new();

    lines = read_file(filename);

    let mut games: Vec<Deck> =  create_games(lines);
   
    for items in &mut games {
        items.check_strength();
    }
    games.sort_by(|a, b| a.strength.cmp(&b.strength));
    
    let mut fvk = Vec::new();
    let mut fok = Vec::new();
    let mut ful = Vec::new();
    let mut thr = Vec::new();
    let mut twp = Vec::new();
    let mut op = Vec::new();
    let mut hc = Vec::new();

    for item in &games {
        match item.strength {
            Strength::FIVEOFKIND => fvk.push(item),
            Strength::FOUROFKIND => fok.push(item),
            Strength::FULLHOUSE => ful.push(item),
            Strength::THREEOFKIND => thr.push(item),
            Strength::TWOPAIR => twp.push(item),
            Strength::ONEPAIR => op.push(item),
            Strength::HIGHCARD => hc.push(item)
        }
    }
    fvk.sort_by(|a, b| a.hand.cmp(&b.hand));
    fok.sort_by(|a, b| a.hand.cmp(&b.hand));
    ful.sort_by(|a, b| a.hand.cmp(&b.hand));
    thr.sort_by(|a, b| a.hand.cmp(&b.hand));
    twp.sort_by(|a, b| a.hand.cmp(&b.hand));
    op.sort_by(|a, b| a.hand.cmp(&b.hand));
    hc.sort_by(|a, b| a.hand.cmp(&b.hand));

    println!("read the file!!");
    // set of avaliable cards (high -> low)
    // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2

    // Game logic
    // Comping by type of hand
    // if hand is same
    // COmpare based on card
    // Larger card first wins

    // Points logic
    // score = bid * rank
    // rank (1 -> N ) weakest to strongest hand
    // total_winnings = sum(bid * rank)

}

fn create_games(lines: Vec<String>) -> Vec<Deck>{
    let mut games: Vec<Deck> = Vec::new();

    for ( _ , value) in lines.iter().enumerate() {
        // parse the value and put it into the deck
        let mut parts = value.split_whitespace();
        if let Some(first) = parts.next() {
            if let Some(second) = parts.next() {
                let hand = first.to_string();
                let bid = second.parse::<i32>().unwrap();
                let mut new_deck: Deck = Deck { 
                    hand: hand, 
                    strength: Strength::HIGHCARD, 
                    bid: bid, rank: 0 };
                games.push(new_deck);
            }
        }
    }
    return games;
}
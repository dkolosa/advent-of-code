use std::collections::HashMap;

fn main() {

    let nth_number = 30000000-1;
                          
    let mut spoken_nums: HashMap<i32, i32> = HashMap::new();
    spoken_nums.insert(9,1);
    spoken_nums.insert(19,2);
    spoken_nums.insert(1,3);
    spoken_nums.insert(6,4);
    spoken_nums.insert(0,5);
    spoken_nums.insert(5,6);



    let mut spoken:Vec<i32> = vec![9,19,1,6,0,5,4];

    let mut idx = spoken.len() as i32;

    let final_num = loop {
        let last_spoken = spoken.last().unwrap().clone();

        if !spoken_nums.contains_key(&last_spoken){
            spoken.push(0);
            spoken_nums.insert(last_spoken.clone(),idx.clone());
        } else {
            let dif_spoken = idx - spoken_nums.get(&last_spoken).unwrap();
            spoken.push(dif_spoken.clone());
            spoken_nums.insert(last_spoken, idx);
        }

        if idx > nth_number {
            break last_spoken;
        }

        idx += 1;
    };

    println!("{} number: {}",nth_number+1, final_num);

}

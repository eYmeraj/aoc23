use std::collections::HashMap;

#[path = "../utils/utils.rs"] mod utils;

fn main(){
    let file_path: &str = "src/../data/input7.txt";
    let content: String = utils::read_file(file_path);
    let data: Vec<String> = content.split("\n")
                                    .map(|s| s.to_string())
                                    .collect();

    part_1(&data);
    part_2(&data);
}

fn hand_power(hand: &str, jolly: bool) -> i64{
    let mut card_counts: HashMap<char, i32> = HashMap::new();
    for card in hand.chars(){
        *card_counts.entry(card).or_insert(0) += 1;
    }
    if jolly && hand.to_string().contains("J"){
        if card_counts.get(&'J').unwrap() < &5{
            let mut max_key = card_counts.iter().max_by_key(|entry| entry.1).unwrap();
            let max_val = max_key.1;
            if *max_key.0 != 'J'{
                card_counts.insert(*max_key.0, max_key.1 + card_counts.get(&'J').unwrap());            }
            else{
                max_key = card_counts.iter()
                                        .max_by_key(|entry| 
                                            if entry.0 != &'J'{
                                            entry.1
                                            }else {
                                            &0
                                            }
                                    ).unwrap();
                card_counts.insert(*max_key.0, max_key.1 + max_val);
            }
            card_counts.insert('J', 0);
        }

    }
    let card_values: Vec<&i32> = card_counts.values().clone().collect();

    if card_values.contains(&&5){
        return 7; 
    }else if card_values.contains(&&4) {
        return 6;
    }else if  card_values.contains(&&3) && card_values.contains(&&2){
        return 5;
    }else if card_values.contains(&&3){
        return 4;
    }else if card_values.contains(&&2) && card_counts.keys().len() == 3{
        return 3;
    }else if card_values.contains(&&2) {
        return 2;
    }else {
        return 1;
    }
}

fn encode_hand(hand: &String, joker_value: i64) -> (i64, i64){
    let pair: Vec<&str> = hand.split(" ")
                                    .collect();
    let bid: i64 = pair[1].parse().unwrap();
    let mut hand_num: i64 = 0;

    for (idx, card) in pair[0].char_indices(){
        let power: u32 = 8_u32 - 2_u32*(idx as u32);
        let factor: i64 = 10_i64.pow(power);
        let card_val: i64;
        if card.is_digit(10){
            card_val = card.to_string().parse::<i64>().unwrap();
            hand_num += factor * card_val;
        }else {
            match card {
                'A' => card_val = 14_i64,
                'K' => card_val = 13_i64,
                'Q' => card_val = 12_i64,
                'J' => card_val = joker_value,
                'T' => card_val = 10_i64,
                _ => todo!()
            }
            hand_num += factor * card_val;
        }

    }
    let mut part_2: bool = false;
    if joker_value == 1{
        part_2 = true;
    }
    let power = hand_power(pair[0], part_2);
    hand_num += power * (10_i64.pow(10_u32));
    return (hand_num, bid);
}

fn argsort(data: &Vec<i64>) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| data[i]);
    return indices;
}
fn part_1(data: &Vec<String>){
    let mut hands: Vec<i64> = vec![];
    let mut bids: Vec<i64> = vec![];
    for hand in data.iter(){
        let (hand_n, bid): (i64, i64) = encode_hand(hand, 11);
        hands.push(hand_n);
        bids.push(bid);
    }

    let sorting_indeces = argsort(&hands);
    let mut tot: i64 = 0;
    let mut counter: i64 = 1;
    for idx in sorting_indeces{
        tot += bids[idx] * counter;
        counter += 1;
    }
    println!("{}", tot);
}

fn part_2(data: &Vec<String>){
    let mut hands: Vec<i64> = vec![];
    let mut bids: Vec<i64> = vec![];
    for hand in data.iter(){
        let (hand_n, bid): (i64, i64) = encode_hand(hand, 1);
        hands.push(hand_n);
        bids.push(bid);
    }

    let sorting_indeces = argsort(&hands);
    let mut tot: i64 = 0;
    let mut counter: i64 = 1;
    for idx in sorting_indeces{
        tot += bids[idx] * counter;
        counter += 1;
    }
    println!("{}", tot);
}
use std::collections::HashMap;

#[path = "../utils/utils.rs"]
mod utils;
fn main() {
    use std::time::Instant;
    let mut now = Instant::now();
    let file_path: &str = "src/../data/input15.txt";
    let content: String = utils::read_file(file_path);
    let data: Vec<String> = content.split(",").map(|s| s.to_string()).collect();
    part_1(&data);
    println!("Part 1 time: {:.3?}", now.elapsed());
    now = Instant::now();
    part_2(&data);
    println!("Part 2 time: {:.3?}", now.elapsed());
}

fn part_1(data: &Vec<String>) {
    let mut tot: u32 = 0;
    for element in data.iter() {
        tot += string_decoder(element);
    }

    println!("{}", tot);
}

fn part_2(data: &Vec<String>) {
    let mut stack_codes: Vec<String> = vec![];
    let mut stack_values: Vec<u32> = vec![];

    for element in data.iter() {
        if element.contains("=") {
            let tmp_vec: Vec<String> = element.split("=").map(|s| s.to_string()).collect();
            let code: String = tmp_vec[0].clone();
            let value_str: String = tmp_vec[1].clone();
            let value: u32 = value_str.parse::<u32>().unwrap();
            if stack_codes.contains(&code) {
                let idx = stack_codes.iter().position(|r| r == &code).unwrap();
                stack_values[idx] = value;
            } else {
                stack_codes.push(code);
                stack_values.push(value);
            }
        } else if element.contains("-") {
            let code = element.replace("-", "");
            if stack_codes.contains(&code) {
                let idx = stack_codes.iter().position(|r| r == &code).unwrap();
                stack_codes.remove(idx);
                stack_values.remove(idx);
            }
        }
    }
    let mut box_amount_counter: HashMap<u32, u32> = HashMap::new();
    for i in 1..=256 {
        box_amount_counter.insert(i as u32, 1);
    }
    let mut tot: u32 = 0;
    for (code, value) in stack_codes.iter().zip(stack_values) {
        let current_box: u32 = string_decoder(code) + 1;
        let current_val: u32 = box_amount_counter.get(&current_box).unwrap().clone();
        box_amount_counter.insert(current_box, current_val + 1);
        tot += current_box * current_val * value;
    }
    println!("{}", tot)
}

fn string_decoder(string: &String) -> u32 {
    let mut current_val: u32 = 0;
    let multiplier: u32 = 17;
    let modulo: u32 = 256;
    for character in string.chars() {
        let val: u32 = character as u32;
        current_val += val;
        current_val *= multiplier;
        current_val = current_val % modulo;
    }
    return current_val;
}

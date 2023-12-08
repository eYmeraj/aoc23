#[path = "../utils/utils.rs"] mod utils;
use std::collections::HashMap;
use std::cmp::{max, min};

fn main(){
    let file_path: &str = "src/../data/input8.txt";
    let content: String = utils::read_file(file_path);
    let data: Vec<String> = content.split("\n")
                                    .map(|s| s.to_string())
                                    .collect();
    part_1(&data);
    part_2(&data);
}

fn part_1(data: &Vec<String>){
    let mut rules: Vec<i32> = data[0].replace("L", "0").replace("R", "1").as_str().chars().map(|i| i.to_string().parse::<i32>().unwrap()).collect();
    let map = get_map(data);
    let mut current: Vec<&str> = vec![];
    for val in map.keys().into_iter(){
        if val == &"AAA"{
            current.push(val);
        }
    }
    let mut tot: i64 = 0;
    let n = current.len();

    loop {
        tot += 1;
        let dir = rules[0];
        rules.remove(0);
        rules.push(dir);
        let mut tmp_counter = 0;
        let tmp_current = current.clone();
        for (idx, val) in tmp_current.iter().enumerate(){
            
            if dir == 1{
                current[idx] = map.get(val).unwrap().1;
            }else {
                current[idx] = map.get(val).unwrap().0;
            }
            if current[idx] == "ZZZ" {
                tmp_counter += 1;
            }
        }
        if tmp_counter == n {break}
    }
    println!("{}", tot)
}
fn part_2(data: &Vec<String>){
    let mut rules: Vec<i32> = data[0].replace("L", "0").replace("R", "1").as_str().chars().map(|i| i.to_string().parse::<i32>().unwrap()).collect();
    let map = get_map(data);
    let mut current: Vec<&str> = vec![];
    for val in map.keys().into_iter(){
        if val.ends_with("A"){
            current.push(val);
        }
    }
    let mut tot: i64 = 0;
    let n = current.len();
    let mut mmcs = vec![0; n];
    while mmcs.contains(&0){
        tot += 1;
        let dir = rules[0];
        rules.remove(0);
        rules.push(dir);
        let tmp_current = current.clone();
        for (idx, val) in tmp_current.iter().enumerate(){
            if dir == 1{
                current[idx] = map.get(val).unwrap().1;
            }else {
                current[idx] = map.get(val).unwrap().0;
            }
            if current[idx].ends_with("Z") {
                if mmcs[idx] == 0{
                    mmcs[idx] = tot;
                }
            }
        }
    }
    let mut sol: usize = 1;
    while mmcs.len() > 0{
        let tmp_num = mmcs.pop().unwrap();
        sol = lcm(sol, tmp_num as usize);
    }
    println!("{}", sol)
}


fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn get_map(data: &Vec<String>) -> HashMap<&str, (&str, &str)>{
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let n = data.len();
    for i in 2..n{
        let row = data[i].as_str();
        let key = &row[..3];
        let value = (&row[7..10], &row[12..15]);
        map.insert(key, value);
    }
    return map;
}
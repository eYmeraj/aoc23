#[path = "../utils/utils.rs"] mod utils;
use std::collections::HashMap;

fn main(){
    let file_path: &str = "src/../data/input4.txt";
    let content: String = utils::read_file(file_path);
    let data: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
    part_1(&data);
    part_2(&data);
}

fn part_1(data: &Vec<String>){
    let mut tot: u32 = 0;
    for i in 0..data.len(){
        let val: String;
        let (winning_nums_str, played_nums_str): (String, String);
        let (winning_nums, played_nums): (Vec<&str>, Vec<&str>);

        val = data[i].split(":").collect::<Vec<&str>>()[1].to_string().replace("  ", " ");
        
        winning_nums_str = val.split("|").collect::<Vec<&str>>()[0].trim().to_string();
        played_nums_str = val.split("|").collect::<Vec<&str>>()[1].trim().to_string();

        winning_nums = winning_nums_str.split(" ").collect();
        played_nums = played_nums_str.split(" ").collect();
        let mut wins: u32 = 0;
        for num in winning_nums.iter(){

            if played_nums.contains(num){
                wins += 1
            }
        }
        if wins > 1 {
            let points = 2_u32.pow(wins - 1);

            tot += points;
        }else{
            tot += wins
        }

        
        
    }
    println!("{}", tot);
}

fn part_2(data: &Vec<String>){
    let mut tot: usize = 0;
    let mut ticket_info: HashMap<usize, usize> = HashMap::new();
    for i in 0..data.len(){
        ticket_info.entry(i).or_insert(1);
    }

    for i in 0..data.len(){
        let val: String;
        let (winning_nums_str, played_nums_str): (String, String);
        let (winning_nums, played_nums): (Vec<&str>, Vec<&str>);

        val = data[i].split(":").collect::<Vec<&str>>()[1].to_string().replace("  ", " ");
        
        winning_nums_str = val.split("|").collect::<Vec<&str>>()[0].trim().to_string();
        played_nums_str = val.split("|").collect::<Vec<&str>>()[1].trim().to_string();

        winning_nums = winning_nums_str.split(" ").collect();
        played_nums = played_nums_str.split(" ").collect();
        let mut wins: i32 = 0;
        for num in winning_nums.iter(){
            if played_nums.contains(num){
                wins += 1
            }
        }
        let mut counter: usize = 1;
        for _ in 0..wins{
            let tmp_val: usize = *ticket_info.get(&(i+counter)).unwrap();
            ticket_info.insert(i + counter, tmp_val + 1 * ticket_info.get(&i).unwrap());
            counter += 1;
        }
    }

    for (_, val) in ticket_info.iter(){
        tot += val;
    }
    println!("{}", tot);
}
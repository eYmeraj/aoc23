#[path = "../utils/utils.rs"] mod utils;
use std::collections::HashMap;

fn main(){
    let file_path: &str = "src/../data/input3.txt";
    let content = utils::read_file(file_path);
    let data: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
    part_1(data.clone());
    part_2(data);
}

fn part_1 (data: Vec<String>) -> (){
    let mut tot: i32 = 0;
    let reg_exp = regex::Regex::new(r"\d+").unwrap();
    for (idx, row) in data.iter().enumerate(){
        for instance in reg_exp.find_iter(row){
            if num_is_valid(data.clone(), idx, instance.start(), instance.end()) {
                tot += instance.as_str().parse::<i32>().unwrap();
            }
        }
    }
    println!("{}", tot)
}

fn part_2 (data: Vec<String>) -> (){
    let mut gears_info: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    let mut tot: i32 = 0;
    let reg_exp = regex::Regex::new(r"\d+").unwrap();
    for (idx, row) in data.iter().enumerate(){
        for instance in reg_exp.find_iter(row){
            let number_found = instance.as_str().parse::<i32>().unwrap();
            gears_info = gear_nums(gears_info, data.clone(), idx, instance.start(), instance.end(), number_found)
        }
    }
    for (_, numbers) in gears_info.iter() {
        if numbers.len() == 2 {
            tot += numbers[0] * numbers[1];
        }
    }
    println!("{}", tot)
}   

fn num_is_valid(data: Vec<String>, row_num: usize, mut col_start: usize, col_end: usize) -> bool{
    let (row_start, row_end) : (usize, usize);
    row_end = row_num + 1;
    if row_num == 0{
        row_start = 0;
    }else {
        row_start = row_num - 1;
    }
    if col_start > 0{
        col_start = col_start - 1;
    }
    for i in row_start..=row_end{
        for j in col_start..=col_end{
            if i < data.len() && j < data[i].len(){
                let val = data[i].chars().nth(j).unwrap();
                if !val.is_numeric() && val != '.'{
                    return true;
                }
            }
        }
    }
    return false;
}


fn gear_nums(mut gears_info: HashMap<(usize, usize), Vec<i32>>, data: Vec<String>, row_num: usize, mut col_start: usize, col_end: usize, number_found: i32) -> HashMap<(usize, usize), Vec<i32>> {
    let (row_start, row_end) : (usize, usize);
    row_end = row_num + 1;
    if row_num == 0{
        row_start = 0;
    }else {
        row_start = row_num - 1;
    }
    if col_start > 0{
        col_start = col_start - 1;
    }
    for i in row_start..=row_end{
        for j in col_start..=col_end{
            if i < data.len() && j < data[i].len(){
                let val = data[i].chars().nth(j).unwrap();
                if val == '*'{
                    gears_info.entry((i, j)).or_insert(vec![]).push(number_found);
                }
            }
        }
    }
    return gears_info;
}
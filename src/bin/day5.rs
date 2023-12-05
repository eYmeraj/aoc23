#[path = "../utils/utils.rs"] mod utils;
use std::cmp;

fn main(){
    let file_path: &str = "src/../data/input5.txt";
    let content: String = utils::read_file(file_path);
    let data: Vec<String> = content.split("\n\n").map(|s| s.to_string()).collect();
    part_1(&data);
    part_2(&data);
}

fn part_1(data: &Vec<String>){
    let reg_exp = regex::Regex::new(r"\d+").unwrap();
    let starting_seeds: Vec<i64> = reg_exp.find_iter(&data[0]).filter_map(|m| m.as_str().parse().ok()).collect();
    let mut tmp_seeds = starting_seeds.clone();
    for conversion in 1..data.len(){
        let mut new_seeds: Vec<i64> = vec![];
        let mappings = &data[conversion];
        let mut source: Vec<std::ops::Range<i64>> = vec![];
        let mut destination: Vec<std::ops::Range<i64>> = vec![];
        let mut modified: bool;
        let mapping_rows: Vec<&str> = mappings.split("\n").collect();
        for row in mapping_rows{
            let hold_nums: Vec<i64> = reg_exp.find_iter(&row).filter_map(|m| m.as_str().parse().ok()).collect();
            if hold_nums.len() > 0{
                source.push(hold_nums[1]..hold_nums[1] + hold_nums[2]);
                destination.push(hold_nums[0]..hold_nums[0] + hold_nums[2]);
            }
        }

        for num in tmp_seeds.iter(){
            modified = false;
            for (src_range, dst_range) in source.iter().zip(destination.clone()){
                if src_range.contains(&num){
                    let diff = num - src_range.start;
                    let new_num = dst_range.start + diff;
                    new_seeds.push(new_num);
                    modified = true;
                    break
                }
            }
            if !modified{
                new_seeds.push(*num);
            }
        }
        tmp_seeds = new_seeds
    }
    println!("{:?}", tmp_seeds.iter().min().unwrap())
}

fn part_2(data: &Vec<String>){
    let reg_exp = regex::Regex::new(r"\d+").unwrap();
    let mut starting_seeds: Vec<std::ops::Range<i64>> = vec![];
    let starting_numbers: Vec<i64> = reg_exp.find_iter(&data[0]).filter_map(|m| m.as_str().parse().ok()).collect();
    let n = starting_numbers.len();
    let mut i: usize = 0;
    while i < n{
        starting_seeds.push(starting_numbers[i]..starting_numbers[i]+starting_numbers[i+1]);
        i += 2
    }

    let mut tmp_seeds = starting_seeds.clone();
    for conversion in 1..data.len(){
        let mut new_seeds: Vec<std::ops::Range<i64>> = vec![];
        let mappings = &data[conversion];
        let mut source: Vec<std::ops::Range<i64>> = vec![];
        let mut destination: Vec<std::ops::Range<i64>> = vec![];
        let mut modified: bool;
        let mapping_rows: Vec<&str> = mappings.split("\n").collect();
        for row in mapping_rows{
            let hold_nums: Vec<i64> = reg_exp.find_iter(&row).filter_map(|m| m.as_str().parse().ok()).collect();
            if hold_nums.len() > 0{
                source.push(hold_nums[1]..hold_nums[1] + hold_nums[2]);
                destination.push(hold_nums[0]..hold_nums[0] + hold_nums[2]);
            }
        }
        
        while tmp_seeds.len() > 0{
            modified = false;
            let current_range = tmp_seeds.pop().unwrap();
            for (src_range, dst_range) in source.iter().zip(destination.clone()){
                if (src_range.start > current_range.end - 1) || (src_range.end - 1 < current_range.start){
                    continue;
                }else{
                    let intersection_range = cmp::max(current_range.start, src_range.start)..cmp::min(current_range.end, src_range.end);
                    if intersection_range.start > current_range.start{
                        let diff = intersection_range.start - current_range.start;
                        tmp_seeds.push(current_range.start..current_range.start + diff)
                    }
                    if intersection_range.end < current_range.end{
                        let diff = current_range.end - intersection_range.end;
                        tmp_seeds.push(intersection_range.end..intersection_range.end + diff);
                    }
                    modified = true;
                    let idx_start = intersection_range.start - src_range.start;
                    let idx_end = intersection_range.end - src_range.start;
                    new_seeds.push(dst_range.start+idx_start..dst_range.start+idx_end);
                    break;
                }
            }
            if !modified{
                new_seeds.push(current_range);
            }
        }
        tmp_seeds = new_seeds
    }
    println!("{:?}", tmp_seeds.iter().map(|x| x.start).collect::<Vec<i64>>().iter().min().unwrap());
}
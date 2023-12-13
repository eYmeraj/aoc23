use cached::proc_macro::cached;
use cached::SizedCache;

#[path = "../utils/utils.rs"] mod utils;

fn main() {
    let file_path: &str = "src/../data/input12.txt";
    let content: String = utils::read_file(file_path);
    let data: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
    part_1(&data);
    part_2(&data);
}

fn part_1(data: &Vec<String>) {
    let mut lines: Vec<String> = vec![];
    let mut solutions: Vec<Vec<i32>> = vec![];

    for row in data.iter(){
        let mut tmp_pair: Vec<String> = row.split(" ").map(|s| s.to_string()).collect();
        tmp_pair[0].push_str(".");
        let group_line = tmp_pair[0].clone();
        let reg_exp = regex::Regex::new(r"\d+").unwrap();
        let groups_num: Vec<i32> = reg_exp.find_iter(&tmp_pair[1]).filter_map(|m| m.as_str().parse::<i32>().ok()).collect();
        solutions.push(groups_num);
        lines.push(group_line);
    }
    let mut tot: i64 = 0;
    for (line, sol) in lines.iter().zip(solutions){
        tot += count_permutations(line, sol, 0)
    }
    println!("{:?}", tot);
}
fn part_2(data: &Vec<String>) {
    let mut lines: Vec<String> = vec![];
    let mut solutions: Vec<Vec<i32>> = vec![];

    for row in data.iter(){
        let tmp_pair: Vec<String> = row.split(" ").map(|s| s.to_string()).collect();
        let mut group_line = tmp_pair[0].clone();
        let reg_exp = regex::Regex::new(r"\d+").unwrap();
        let mut groups_num: Vec<i32> = reg_exp.find_iter(&tmp_pair[1]).filter_map(|m| m.as_str().parse::<i32>().ok()).collect();
        let og_group_nums = groups_num.clone();
        for _ in 0..4{
            groups_num.extend(&og_group_nums);
            group_line.push_str("?");
            group_line.push_str(&tmp_pair[0]);
        }
        group_line.push_str(".");
        
        solutions.push(groups_num);
        lines.push(group_line);
    }

    let mut tot: i64 = 0;
    for (line, sol) in lines.iter().zip(solutions){
        let val = count_permutations(line, sol, 0);
        tot += val;        
    }
    println!("{:?}", tot);
}
#[cached(
    type = "SizedCache<(String, Vec<i32>, i32), i64>",
    create = "{ SizedCache::with_size(100000)}",
)]
fn count_permutations(line: &String, solution: Vec<i32>, count: i32) -> i64{
    if line.len() == 0{
        if solution.len() == 0 && count == 0{
            return 1_i64;
        }else{
            return 0_i64;
        }
    }
    let current_val = line.chars().nth(0).unwrap();
    let mut choices: Vec<char> = vec![];
    let mut res: i64 = 0;
    if current_val == '?'{
        choices.push('#');
        choices.push('.');
    }else{
        choices.push(current_val);
    }
    let mut sub_string_chars = line.chars();
    sub_string_chars.next();
    let sub_string: &String = &sub_string_chars.as_str().to_string().clone();
    let mut sub_solution: Vec<i32> = solution.clone();
    if !sub_solution.is_empty(){
        sub_solution.remove(0);
    }
    let current_group: &i32;
    if solution.is_empty(){
        current_group = &-1;
    }else{
        current_group = &solution[0];
    }
    for val in choices.iter(){
        if val == &'#' {
            if &count < current_group{
                res += count_permutations(sub_string, solution.clone(), count + 1);
            }
        }else{
            if count > 0{
                if &count == current_group && !solution.is_empty(){
                    res += count_permutations(sub_string, sub_solution.clone(), 0)
                }
            }else{
                res += count_permutations(sub_string, solution.clone(), 0)
            }   
        }
    }
    return res;
}
#[path = "../utils/utils.rs"] mod utils;

fn main(){
    let file_path: &str = "src/../data/input2.txt";
    let content = utils::read_file(file_path);
    let data: Vec<&str> = content.split("\n").collect();
    part_1(data.clone());
    part_2(data);
}

fn part_1 (data: Vec<&str>) -> (){
    let mut tot: i64 = 0;
    let (max_r, max_g, max_b): (i64, i64, i64) = (12, 13, 14);
    let mut idx: i64 = 0;
    let mut max_val: i64;
    for row in data{
        idx += 1;
        let mut valid: bool = true;
        for color in ["blue", "green", "red"]{
            let partial_str: Vec<&str> = row.split(color).collect();
            match color {
                "blue" => max_val = max_b,
                "red" => max_val = max_r,
                "green" => max_val = max_g,
                &_ => todo!(),
            }
            let n = partial_str.len();
            for sub_partial in &partial_str[0..n - 1]{
                let num_array : Vec<&str> = sub_partial
                                            .split(" ")
                                            .collect();
                let num: i64 = num_array[num_array.len() - 2].parse::<i64>().unwrap();
                if num > max_val{
                    valid = false;
                    break
                }
            }
            
        }
        if valid {
            tot += idx
        }
        
    }
    println!("{}", tot)
}

fn part_2 (data: Vec<&str>) -> (){
    let mut tot: i64 = 0;
    // let (max_r, max_g, max_b): (i64, i64, i64) = (12, 13, 14);
    let mut current_max: i64;
    let mut tmp_val: i64;
    for row in data{
        // blue
        tmp_val = 1;
        // let mut valid: bool = true;
        for color in ["blue", "green", "red"]{
            current_max = 1;
            let partial_str: Vec<&str> = row.split(color).collect();
            let n = partial_str.len();
            for sub_partial in &partial_str[0..n - 1]{
                let num_array : Vec<&str> = sub_partial
                                            .split(" ")
                                            .collect();
                let num: i64 = num_array[num_array.len() - 2].parse::<i64>().unwrap();
                
                if num > current_max{
                    current_max = num;
                }
            }
            tmp_val *= current_max;
            
        }
        tot += tmp_val;
    }
    println!("{}", tot)
}
#[path = "../utils/utils.rs"] mod utils;

fn main(){
    let file_path: &str = "src/../data/input6.txt";
    let content: String = utils::read_file(file_path);
    let data: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
    part_1(&data);
    part_2(&data);
}

fn part_1(data: &Vec<String>){
    let reg_exp = regex::Regex::new(r"\d+").unwrap();
    let times: Vec<i64> = reg_exp.find_iter(&data[0]).filter_map(|m| m.as_str().parse().ok()).collect();
    let records: Vec<i64> = reg_exp.find_iter(&data[1]).filter_map(|m| m.as_str().parse().ok()).collect();
    let mut tot: i64 = 1;
    for (t, r) in times.iter().zip(records){
        let res = solve_quadratic_int(*t, r);
        tot *= res;
    }
    println!("{}", tot);
}

fn part_2(data: &Vec<String>){
    let reg_exp = regex::Regex::new(r"\d+").unwrap();
    let times: Vec<i64> = reg_exp.find_iter(&data[0].replace(" ", "")).filter_map(|m| m.as_str().parse().ok()).collect();
    let records: Vec<i64> = reg_exp.find_iter(&data[1].replace(" ", "")).filter_map(|m| m.as_str().parse().ok()).collect();
    let mut tot: i64 = 1;
    for (t, r) in times.iter().zip(records){
        let res = solve_quadratic_int(*t, r);
        tot *= res;
    }
    println!("{}", tot);
}


fn solve_quadratic_int(t: i64, r: i64) -> i64{
    let delta: i64 = t.pow(2) - 4*r;
    let delta_sqrt: f64 = (delta as f64).sqrt();
    let (y1, y2): (i64, i64);
    y1 = (((t as f64) + delta_sqrt)/2.0).ceil() as i64;
    y2 = (((t as f64) - delta_sqrt)/2.0).ceil() as i64;
    return y1 - y2;
}
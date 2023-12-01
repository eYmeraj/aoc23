use std::borrow::Cow;

#[path = "../utils/utils.rs"] mod utils;

fn main(){
    let file_path: &str = "src/../data/input1.txt";
    let content = utils::read_file(file_path);
    let data: Vec<&str> = content.split("\n").collect();
    part_1(data.clone());
    part_2(data);
}

fn part_1 (data: Vec<&str>) -> (){
    let mut sum: i64 = 0;
    for row in data{
        let mut res: String = row.chars().filter(|c| c.is_digit(10)).collect();
        let i =  res.len() - 1;
        if i == 0 {
            let tmp_res = res.clone();
            res.push_str(&tmp_res);
            sum += res.parse::<i64>().unwrap();
        }else {
            let mut first_chaar = res.chars().nth(0).unwrap().to_string().to_owned();
            let last_chaar = res.chars().nth(i).unwrap().to_string().to_owned();
            first_chaar.push_str(&last_chaar);
            sum += first_chaar.parse::<i64>().unwrap();
        }

    }
    println!("{}", sum);
}

fn part_2 (data: Vec<&str>) -> (){
    let  mut sum: i64 = 0;
    for  row in data{
        let mut tmp = Cow::from(row);
        for (from, to) in [("eightwo", "82"),
                            ("eighthree", "83"),
                            ("nineight", "98"),
                            ("oneight","18"),
                            ("threeight", "38"),
                            ("fiveight", "58"),
                            ("twone", "21"),
                            ("one", "1"),
                            ("two", "2"),
                            ("three", "3"),
                            ("four", "4"),
                            ("five", "5"),
                            ("six", "6"),
                            ("seven", "7"),
                            ("eight", "8"),
                            ("nine", "9")]{
                                tmp = tmp.replace(from, to).into();
        }
        let row = tmp.to_string();
        let mut res: String = row.chars().filter(|c| c.is_digit(10)).collect();
        let i =  res.len() - 1;
        if i == 0 {
            let tmp_res = res.clone();
            res.push_str(&tmp_res);
            sum += res.parse::<i64>().unwrap();
        }else {
            let mut first_chaar = res.chars().nth(0).unwrap().to_string().to_owned();
            let last_chaar = res.chars().nth(i).unwrap().to_string().to_owned();
            first_chaar.push_str(&last_chaar);
            sum += first_chaar.parse::<i64>().unwrap();
        }

    }
    println!("{}", sum);
}
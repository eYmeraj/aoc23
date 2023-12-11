#[path = "../utils/utils.rs"] mod utils;
use std::cmp;
fn main() {
    let file_path: &str = "src/../data/input11.txt";
    let content: String = utils::read_file(file_path);
    let data: Vec<String> = content.split("\n").map(|s| s.to_string().replace(".", "0").replace("#", "1")).collect();
    part_1(&data);
    part_2(&data);
}

fn part_1(data: &Vec<String>) {
    let mut to_dupe_rows: Vec<i32> = vec![];
    let mut to_dupe_cols: Vec<i32> = vec![];
    for (idx, row) in data.iter().enumerate(){
        if !row.contains("1"){
            to_dupe_rows.push(idx.try_into().unwrap());
        }
    }
    for idx in 0..data[0].len(){
        let mut add_col = true;
        for row in data.iter(){
            if row.chars().nth(idx).unwrap() == '1'{
                add_col = false;
                break
            }
        }
        if add_col {
            to_dupe_cols.push(idx.try_into().unwrap());       
        }
    }

    let mut points: Vec<(i32, i32)> = vec![];

    for (i, row) in data.iter().enumerate(){
        for (j, val) in row.chars().enumerate(){
            if val == '1'{
                points.push((i as i32, j as i32));
            }
        }
    }

    let dupe_amout: i32 = 2;
    let mut tot: i32 = 0;
    for i in 0..points.len() - 1{
        let p1 = points[i];
        for j in i..points.len() {
            let p2 = points[j];
            let mut tmp_dist = manhattan_distance(p1, p2);
            let min_row = cmp::min(p1.0, p2.0);
            let max_row = cmp::max(p1.0, p2.0);
            let min_col = cmp::min(p1.1, p2.1);
            let max_col = cmp::max(p1.1, p2.1);
            for dupe_row in to_dupe_rows.iter(){
                if min_row < *dupe_row && max_row > *dupe_row{
                    tmp_dist += dupe_amout -1;
                }
            }

            for dupe_col in to_dupe_cols.iter(){
                if min_col < *dupe_col && max_col > *dupe_col{
                    tmp_dist += dupe_amout -1;
                }
            }
            tot += tmp_dist;
        }
    }
    println!("{}", tot)

    // println!("{:?}", points);
    
}

fn part_2(data: &Vec<String>) {
    let mut to_dupe_rows: Vec<i32> = vec![];
    let mut to_dupe_cols: Vec<i32> = vec![];
    for (idx, row) in data.iter().enumerate(){
        if !row.contains("1"){
            to_dupe_rows.push(idx.try_into().unwrap());
        }
    }
    for idx in 0..data[0].len(){
        let mut add_col = true;
        for row in data.iter(){
            if row.chars().nth(idx).unwrap() == '1'{
                add_col = false;
                break
            }
        }
        if add_col {
            to_dupe_cols.push(idx.try_into().unwrap());       
        }
    }

    let mut points: Vec<(i32, i32)> = vec![];

    for (i, row) in data.iter().enumerate(){
        for (j, val) in row.chars().enumerate(){
            if val == '1'{
                points.push((i as i32, j as i32));
            }
        }
    }

    let dupe_amout: i64 = 1000000;
    let mut tot: i64 = 0;
    for i in 0..points.len() - 1{
        let p1 = points[i];
        for j in i..points.len() {
            let p2 = points[j];
            let mut tmp_dist = manhattan_distance(p1, p2) as i64;
            let min_row = cmp::min(p1.0, p2.0);
            let max_row = cmp::max(p1.0, p2.0);
            let min_col = cmp::min(p1.1, p2.1);
            let max_col = cmp::max(p1.1, p2.1);
            for dupe_row in to_dupe_rows.iter(){
                if min_row < *dupe_row && max_row > *dupe_row{
                    tmp_dist += dupe_amout -1;
                }
            }

            for dupe_col in to_dupe_cols.iter(){
                if min_col < *dupe_col && max_col > *dupe_col{
                    tmp_dist += dupe_amout -1;
                }
            }
            tot += tmp_dist;
        }
    }
    println!("{}", tot)
}

fn manhattan_distance(p1 : (i32, i32), p2 : (i32, i32)) -> i32{
    let x_diff = (p1.0 - p2.0).abs();
    let y_diff = (p1.1 - p2.1).abs();
    return x_diff + y_diff
}
#[path = "../utils/utils.rs"] mod utils;
fn main() {
    let file_path: &str = "src/../data/input9.txt";
    let content: String = utils::read_file(file_path);
    let data: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
    part_1(&data);
    part_2(&data);
}

fn part_1(data: &Vec<String>) {
    let mut hold: Vec<Vec<i32>>;
    let mut tot: i32 = 0;
    for row in data {
        hold = vec![];
        let tmp_vec: Vec<i32>;
        tmp_vec = row.split(" ").map(|i| i.parse::<i32>().unwrap()).collect();
        hold.push(tmp_vec);
        let mut n = hold.len();
        let mut vec_sum: i32 = 0;
        for val in hold[n - 1].iter(){
            vec_sum += val;
        }
        while vec_sum != 0{
            let mut new_vec: Vec<i32> = vec![];
            for idx in 1..hold[n-1].len(){
                let tmp_val: i32 = hold[n - 1][idx] - hold[n - 1][idx - 1];
                new_vec.push(tmp_val)
            }
            hold.push(new_vec);
            n = hold.len();
            vec_sum = 0;
            for val in hold[n - 1].iter(){
                vec_sum += val;
            }
        }
        hold.reverse();
        let mut val: i32 = 0;
        for i in 1..hold.len(){
            val = hold[i].pop().unwrap() + val;
        }
        tot += val;
    }   
    println!("{}", tot);
}

fn part_2(data: &Vec<String>) {
    let mut hold: Vec<Vec<i32>>;
    let mut tot: i32 = 0;
    for row in data {
        hold = vec![];
        let tmp_vec: Vec<i32>;
        tmp_vec = row.split(" ").map(|i| i.parse::<i32>().unwrap()).collect();
        hold.push(tmp_vec);
        let mut n = hold.len();
        let mut vec_sum: i32 = 0;
        for val in hold[n - 1].iter(){
            vec_sum += val;
        }
        while vec_sum != 0{
            let mut new_vec: Vec<i32> = vec![];
            for idx in 1..hold[n-1].len(){
                let tmp_val: i32 = hold[n - 1][idx] - hold[n - 1][idx - 1];
                new_vec.push(tmp_val)
            }
            hold.push(new_vec);
            n = hold.len();
            vec_sum = 0;
            for val in hold[n - 1].iter(){
                vec_sum += val;
            }
        }
        hold.reverse();
        let mut val: i32 = 0;
        for i in 1..hold.len(){
            val = hold[i][0] - val;
        }
        tot += val;
    }   
    println!("{}", tot);
}

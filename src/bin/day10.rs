#[path = "../utils/utils.rs"] mod utils;
fn main() {
    let file_path: &str = "src/../data/input9.txt";
    let content: String = utils::read_file(file_path);
    let data: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
    
    part_1(&data);
    part_2(&data);
}

fn part_1(data: &Vec<String>) {
}

fn part_2(data: &Vec<String>) {
}
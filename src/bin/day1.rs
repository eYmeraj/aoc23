#[path = "../utils/utils.rs"] mod utils;

fn main(){
    let file_path: &str = "src/../data/day1.txt";
    let content = utils::read_file(file_path);
    println!("{}", content)
}
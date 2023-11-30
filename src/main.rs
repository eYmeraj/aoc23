// use std::env;
#[path = "utils/utils.rs"] mod utils;

fn main() {
    println!("AOC23!");
    let file_path = "data/test.txt";
    let contents = utils::read_file(file_path);
    println!("With text:\n{contents}");
    let this_file = file!();
    print!("{}", this_file)
}

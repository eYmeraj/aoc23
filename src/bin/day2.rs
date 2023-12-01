#[path = "../utils/utils.rs"] mod utils;

fn main(){
    let file_path: &str = "src/../data/input2.txt";
    let content = utils::read_file(file_path);
    let data: Vec<&str> = content.split("\n").collect();
    part_1(data.clone());
    part_2(data);
}

fn part_1 (data: Vec<&str>) -> (){

}

fn part_2 (data: Vec<&str>) -> (){
    
}
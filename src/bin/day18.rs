use std::collections::HashMap;

#[path = "../utils/utils.rs"] mod utils;

fn main() {
    use std::time::Instant;
    let mut now = Instant::now();
    let file_path: &str = "src/../data/input18.txt";
    let content: String = utils::read_file(file_path);
    let data: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();

    part_1(&data);
    println!("Part 1 time: {:.3?}", now.elapsed());
    now = Instant::now();
    part_2(&data);
    println!("Part 2 time: {:.3?}", now.elapsed());
    
}

fn part_1(data: &Vec<String>){
    let mut direction_mapping: HashMap<char, (i32, i32)> = HashMap::new();
    direction_mapping.insert('R', (0, 1));
    direction_mapping.insert('L', (0, -1));
    direction_mapping.insert('U', (-1, 0));
    direction_mapping.insert('D', (1, 0));

    let mut edges: Vec<(i32, i32)> = vec![];

    let mut current_coords: (i32, i32) = (0, 0);
    edges.push(current_coords);
    for row in data.iter(){
        let rules: Vec<String> = row.split(" ").map(|s| s.to_string()).collect();

        let current_direction: char = rules[0].chars().nth(0).unwrap();
        let amount: i32 = rules[1].parse::<i32>() .unwrap();
        let heading: &(i32, i32) = direction_mapping.get(&current_direction).unwrap();

        current_coords = (current_coords.0 + heading.0 * amount, current_coords.1 + heading.1 * amount);

        edges.push(current_coords);
    }

    let mut  boarder_len: i32 = 0;
    for idx in 0..edges.len() - 1{
        let p1: (i32, i32) = edges[idx];
        let p2: (i32, i32) = edges[idx + 1];
        boarder_len += (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
    }
    let area: i32 = utils::irregular_polygon_area_i32(&edges);
    let res: i32;
    res = area + 1 - boarder_len/2 + boarder_len;
    println!("{}", res)
}

fn part_2(data: &Vec<String>){
    let mut direction_mapping: HashMap<char, (i64, i64)> = HashMap::new();
    direction_mapping.insert('R', (0, 1));
    direction_mapping.insert('L', (0, -1));
    direction_mapping.insert('U', (-1, 0));
    direction_mapping.insert('D', (1, 0));
    
    let mut direction_convertion: HashMap<char, char> = HashMap::new();
    direction_convertion.insert('0', 'R');
    direction_convertion.insert('1', 'D');
    direction_convertion.insert('2', 'L');
    direction_convertion.insert('3', 'U');

    let mut edges: Vec<(i64, i64)> = vec![];

    let mut current_coords: (i64, i64) = (0, 0);
    edges.push(current_coords);
    for row in data.iter(){
        let rules: Vec<String> = row.split(" ").map(|s| s.to_string()).collect();
        let mut color: &str = rules[2].as_str();
        color = &color[2..color.len()-1];
        let hexa_number_str: &str = &color[..color.len() - 1];
        let amount: i64 = i64::from_str_radix(hexa_number_str, 16).ok().unwrap();
        let current_direction: char = *direction_convertion.get(&(color.chars().nth(color.len() - 1).unwrap())).unwrap();
        let heading: &(i64, i64) = direction_mapping.get(&current_direction).unwrap();

        current_coords = (current_coords.0 + heading.0 * amount, current_coords.1 + heading.1 * amount);

        edges.push(current_coords);
    }

    let mut  boarder_len: i64 = 0;
    for idx in 0..edges.len() - 1{
        let p1: (i64, i64) = edges[idx];
        let p2: (i64, i64) = edges[idx + 1];
        boarder_len += (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
    }
    let area: i64 = utils::irregular_polygon_area_i64(&edges);
    let res: i64;
    
    // Pick's theroem A = i + b/2 - 1. 
    // A = area
    // i = points inside shape with integer coords
    // b = poins on the edges of the shape with integer coords
    // res counds th points inside and the points on the edges of the figure
    res = area + 1 - boarder_len/2 + boarder_len;
    println!("{}", res)
}
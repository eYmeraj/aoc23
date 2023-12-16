use std::collections::HashMap;

#[path = "../utils/utils.rs"]
mod utils;
fn main() {
    use std::time::Instant;
    let mut now = Instant::now();
    let file_path: &str = "src/../data/input16.txt";
    let content: String = utils::read_file(file_path);
    let data: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
    let mut board: HashMap<(i32, i32), char> = HashMap::new();
    for (row_n, line) in data.iter().enumerate(){
        for (col_n, value) in line.chars().enumerate(){
            board.insert((row_n as i32, col_n as i32), value);
        }
    }
    part_1(&board);
    println!("Part 1 time: {:.3?}", now.elapsed());
    now = Instant::now();
    part_2(&board, &data);
    println!("Part 2 time: {:.3?}", now.elapsed());
}


fn part_1(board: &HashMap<(i32, i32), char>){
    let energised = find_paths(board, 0, -1, (0, 1));
    println!("{}", energised);
}

fn part_2(board: &HashMap<(i32, i32), char>, data: &Vec<String>){
    let mut max_energised:i32 = 0;
    for start_row in 0..data.len(){
        max_energised = max_energised.max(find_paths(board, start_row as i32, -1, (0,1)));
        max_energised = max_energised.max(find_paths(board, start_row as i32, data[0].len() as i32, (0, -1)));
    }
    for start_col in 0..data[0].len(){
        max_energised = max_energised.max(find_paths(board, -1, start_col as i32, (1,0)));
        max_energised = max_energised.max(find_paths(board, data.len() as i32, start_col as i32, (-1,0)));
    }
    println!("{}", max_energised);
}

fn find_paths(board: &HashMap<(i32, i32), char>, start_row: i32, start_column: i32, direction: (i32, i32)) -> i32{
    let possible_directions: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut visited: HashMap<(i32, i32, (i32, i32)), bool> = HashMap::new();
    let mut seen_nodes: HashMap<(i32, i32), bool> = HashMap::new(); 
    let mut stack: Vec<(i32, i32, (i32, i32))> = vec![];
    stack.push((start_row, start_column, direction));
    while stack.len() > 0 {
        let (mut row, mut col, mut direction) :(i32, i32, (i32, i32)) = stack.pop().unwrap();
        loop {
            visited.insert((row, col, direction), true);
            seen_nodes.insert((row, col), true);
            row += direction.0;
            col += direction.1;
            let value: &char;
            if board.get(&(row, col)) != None{
                value = board.get(&(row, col)).unwrap();
            }else{
                break;
            }

            if *value == '\\'{
                if direction == possible_directions[0]{
                    direction = possible_directions[2];
                }else if direction == possible_directions[1]{
                    direction = possible_directions[3];
                }else if direction == possible_directions[2]{
                    direction = possible_directions[0];
                }else if direction == possible_directions[3]{
                    direction = possible_directions[1];
                }
            }else if *value == '/'{
                if direction == possible_directions[0]{
                    direction = possible_directions[3];
                }else if direction == possible_directions[1]{
                    direction = possible_directions[2];
                }else if direction == possible_directions[2]{
                    direction = possible_directions[1];
                }else if direction == possible_directions[3]{
                    direction = possible_directions[0];
                }
            }else if *value == '|'{
                if direction == possible_directions[0] || direction == possible_directions[1]{
                    direction = possible_directions[2];
                    if visited.get(&(row, col, possible_directions[3])) == None{
                        stack.push((row, col, possible_directions[3]));
                    }
                    if visited.get(&(row, col, direction)) != None{
                        break
                    }
                }
            }else if *value == '-'{
                if direction == possible_directions[2] || direction == possible_directions[3]{
                    direction = possible_directions[0];
                    if visited.get(&(row, col, possible_directions[1])) == None{
                        stack.push((row, col, possible_directions[1]));
                    }
                    if visited.get(&(row, col, direction)) != None{
                        break
                    }
                }
            }else if *value == '.'{
                continue;
            }
        }
    }
    return seen_nodes.len() as i32 - 1; 
}
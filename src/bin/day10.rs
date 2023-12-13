#[path = "../utils/utils.rs"] mod utils;
fn main() {
    let file_path: &str = "src/../data/input10.txt";
    let content: String = utils::read_file(file_path);
    let mut data: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
    part_1(&data);
    part_2(&mut data);
}

fn part_1(data: &Vec<String>) {
    
    let path: Vec<(usize, usize)>;
    path = find_path(data);
    let max_dist: i32 = path.len().try_into().unwrap();

    println!("{}", max_dist/2)
}

fn part_2(data: &mut Vec<String>) {
    let mut not_path: Vec<(usize, usize)> = vec![];
    let starting_char: String = String::from(find_starting_char(data));
    
    let path: Vec<(usize, usize)>;
    path = find_path(data);
    for (i, row) in data.iter().enumerate(){
        for (j, _) in row.chars().enumerate(){
            if !path.contains(&(i, j)){
                not_path.push((i, j));
            }
        }
    }

    for dot in not_path{
        data[dot.0].replace_range(dot.1..dot.1+1, ".");
    }
    let mut tot_points: u32 = 0;
    for (_, row) in data.iter().enumerate(){
        let tmp_row = row.clone();
        for (j, val) in row.chars().enumerate(){
            let mut play_row = tmp_row.clone();
            let tot_walls: u32;
            if val == '.'{
                play_row = play_row[j..].to_string();
                if play_row.len() > 0 {
                    play_row = play_row.replace(".", "");
                    play_row = play_row.replace("S", &starting_char);
                    play_row = play_row.replace("-", "");
                    play_row = play_row.replace("|", "1");
                    play_row = play_row.replace("F7", "2");
                    play_row = play_row.replace("FJ", "1");
                    play_row = play_row.replace("LJ", "2");
                    play_row = play_row.replace("L7", "1");
                    if play_row.len() > 0 {
                        tot_walls = play_row.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>();
                        if tot_walls % 2 == 1{
                            tot_points += 1
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", tot_points)
}



fn find_next_coords(value: char, current_coords: (usize, usize), old_coords: (usize, usize)) -> (usize, usize){
    let mut possible_coords: Vec<(usize, usize)> = vec![];
    if value == '|'{
        possible_coords.push((current_coords.0 + 1, current_coords.1));
        possible_coords.push((current_coords.0 - 1, current_coords.1));
    } else if value == '-'{
        possible_coords.push((current_coords.0, current_coords.1 + 1));
        possible_coords.push((current_coords.0, current_coords.1 - 1));
    } else if value == 'L' {
        possible_coords.push((current_coords.0, current_coords.1 + 1));
        possible_coords.push((current_coords.0 - 1, current_coords.1));
    }else if value == 'F' {
        possible_coords.push((current_coords.0, current_coords.1 + 1));        
        possible_coords.push((current_coords.0 + 1, current_coords.1));
    }else if value == '7' {
        possible_coords.push((current_coords.0, current_coords.1 - 1));
        possible_coords.push((current_coords.0 + 1, current_coords.1));
    }else if value == 'J' {
        possible_coords.push((current_coords.0, current_coords.1 - 1));
        possible_coords.push((current_coords.0 - 1, current_coords.1));
    }
    for new_coords in possible_coords.iter(){
        if new_coords != &old_coords{
            return *new_coords;
        }
    }
    return old_coords;
}


fn find_starting_char(data: &Vec<String>) -> char{
    let mut starting_point: (usize, usize) = (0, 0);
    'outer: for (i, row) in data.iter().enumerate(){
        for (j, val) in row.chars().enumerate(){
            if val == 'S'{
                starting_point = (i, j);
                break 'outer;
            }
        }
    }
    let possibble_choices: [char; 6] = ['F', 'L', 'J', '7', '-', '|'];
    for val in possibble_choices{
        let connections = find_connections(val, starting_point);
        let mut tot = 0;
        for neigh in connections{
            let neigh_val: char = data[neigh.0].chars().nth(neigh.1).unwrap();
            let neigh_connections = find_connections(neigh_val, neigh);
            if neigh_connections.contains(&starting_point){
                tot += 1
            }else{ break }
        }
        if tot == 2{
            return val;
        }
    }
    return ' ' ; 
}

fn find_path(data: &Vec<String>) -> Vec<(usize, usize)>{
    let mut starting_point: (usize, usize) = (0, 0);
    'outer: for (i, row) in data.iter().enumerate(){
        for (j, val) in row.chars().enumerate(){
            if val == 'S'{
                starting_point = (i, j);
                break 'outer;
            }
        }
    }

    let mut path: Vec<(usize, usize)> = vec![];
    
    // this is hard coded
    let mut old_coords: (usize, usize) = starting_point;
    let starting_char: char = find_starting_char(data);
    let starting_connections = find_connections(starting_char, old_coords);
    let mut current_coords: (usize, usize)= starting_connections[0];

    path.push(current_coords);
    while current_coords != starting_point{
        let tmp_current_coords = current_coords.clone();
        let current_val: char = data[current_coords.0].chars().nth(current_coords.1).unwrap();
        current_coords = find_next_coords(current_val, current_coords, old_coords);
        old_coords = tmp_current_coords;
        path.push(current_coords);
    }
    return path;
}

fn find_connections(value: char, current_coords: (usize, usize)) -> Vec<(usize, usize)>{
    let mut possible_coords: Vec<(usize, usize)> = vec![];
    if value == '|'{
        possible_coords.push((current_coords.0 + 1, current_coords.1));
        possible_coords.push((current_coords.0 - 1, current_coords.1));
    } else if value == '-'{
        possible_coords.push((current_coords.0, current_coords.1 + 1));
        possible_coords.push((current_coords.0, current_coords.1 - 1));
    } else if value == 'L' {
        possible_coords.push((current_coords.0, current_coords.1 + 1));
        possible_coords.push((current_coords.0 - 1, current_coords.1));
    }else if value == 'F' {
        possible_coords.push((current_coords.0, current_coords.1 + 1));        
        possible_coords.push((current_coords.0 + 1, current_coords.1));
    }else if value == '7' {
        possible_coords.push((current_coords.0, current_coords.1 - 1));
        possible_coords.push((current_coords.0 + 1, current_coords.1));
    }else if value == 'J' {
        possible_coords.push((current_coords.0, current_coords.1 - 1));
        possible_coords.push((current_coords.0 - 1, current_coords.1));
    }
    return possible_coords;
}
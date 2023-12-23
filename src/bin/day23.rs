use std::{collections::{HashMap, HashSet}};

#[path = "../utils/utils.rs"] mod utils;

fn main() {
    use std::time::Instant;
    let mut now = Instant::now();
    let file_path: &str = "src/../data/input23.txt";
    let content: String = utils::read_file(file_path);
    let data: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
    let board: Vec<Vec<char>> = parse_input(data);
    part_1(&board);
    println!("Part 1 time: {:.3?}", now.elapsed());
    now = Instant::now();
    part_2(&board);
    println!("Part 2 time: {:.3?}", now.elapsed());
    
}

fn part_1(board: &Vec<Vec<char>>){
    let start_point: (i32, i32) = (0, 1);
    let is_slippery: bool = true;
    let end_point: (i32, i32) = ((board.len()-1).try_into().unwrap(), (board[0].len() - 2).try_into().unwrap());
    let graph: HashMap<(i32, i32), Vec<(i32, i32, i32)>> = find_junctions(&start_point, &end_point, board, is_slippery);
    let res: i32;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(start_point.clone());
    res = find_longest_path(start_point, &end_point, &graph, visited, 0, 0);
    println!("{}", res)
}

fn part_2(board: &Vec<Vec<char>>){
    let start_point: (i32, i32) = (0, 1);
    let is_slippery: bool = false;
    let end_point: (i32, i32) = ((board.len()-1).try_into().unwrap(), (board[0].len() - 2).try_into().unwrap());
    let graph: HashMap<(i32, i32), Vec<(i32, i32, i32)>> = find_junctions(&start_point, &end_point, board, is_slippery);
    let res: i32;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(start_point.clone());
    res = find_longest_path(start_point, &end_point, &graph, visited, 0, 0);
    println!("{}", res)
}

fn find_longest_path(current: (i32, i32), end_point: &(i32, i32), graph: &HashMap<(i32, i32), Vec<(i32, i32, i32)>>, mut visited: HashSet<(i32, i32)>, distance: i32, mut max_found: i32) -> i32{
    if &current == end_point{
        return distance.max(max_found);
    }

    let possible_options: &Vec<(i32, i32, i32)> = graph.get(&current).clone().unwrap();
    visited.insert(current.clone());


    for point in possible_options{
        let tmp_visited = visited.clone();
        let point_coords = (point.0, point.1);
        if !visited.contains(&point_coords){
            let point_dist = point.2;
            max_found = find_longest_path(point_coords, end_point, graph, tmp_visited, distance + point_dist, max_found)
        }
    }
    return max_found;
}

fn parse_input(data: Vec<String>) -> Vec<Vec<char>>{
    let mut board: Vec<Vec<char>> = vec![];
    for row in data.iter(){
        let row_char = row.chars().collect::<Vec<char>>();
        board.push(row_char)
    }
    return board;
}  

fn find_junctions(start_point: &(i32, i32), end_point: &(i32, i32), board: &Vec<Vec<char>>, is_slippery: bool) -> HashMap<(i32, i32), Vec<(i32, i32, i32)>>{
    let mut intersections: HashMap<(i32, i32), Vec<(i32, i32, i32)>>= HashMap::new();
    let tmp_vec_coord: Vec<(i32, i32, i32)> = vec![];
    intersections.insert(start_point.clone(), tmp_vec_coord.clone());
    intersections.insert(end_point.clone(), tmp_vec_coord.clone());
    let mut inter_vec: Vec<(i32, i32)> = vec![];
    inter_vec.push(start_point.clone());
    inter_vec.push(end_point.clone());

    for i in 0..board.len(){
        for j in 0..board[0].len(){
            let value = board[i][j];
            if value == '#'{continue;}
            let tmp_neighs = find_neighbors(&(i as i32, j as i32), board, is_slippery);
            if tmp_neighs.len() > 2{
                intersections.insert((i as i32, j as i32), tmp_vec_coord.clone());
                inter_vec.push((i as i32, j as i32));
            }
        }
    }

    for inter in inter_vec.iter(){
        find_path_to_interesections(inter, board, &mut intersections, is_slippery);
    }

    return intersections;

}

fn find_path_to_interesections(point: &(i32, i32), board: &Vec<Vec<char>>, intersections: &mut HashMap<(i32, i32), Vec<(i32, i32, i32)>>, is_slippery: bool){
    let neighbors = find_neighbors(point, board, is_slippery);
    let mut closest_neigh: Vec<(i32, i32, i32)> = vec![];

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(point.clone());
    for ngs in neighbors.iter(){
        let mut tmp_visited = visited.clone();
        let mut current_neigh = ngs.clone();
        tmp_visited.insert(current_neigh.clone());
        let mut distance = 1;
        'outer: loop {
            let next_neighs = find_neighbors(&current_neigh, board, false);
            for tmp_ngs in next_neighs.iter(){
                if tmp_visited.contains(tmp_ngs){
                    continue;
                }
                tmp_visited.insert(tmp_ngs.clone());
                if is_slippery{
                    let val = board[tmp_ngs.0 as usize][tmp_ngs.1 as usize];
                    if (val == '>' && current_neigh.1 > tmp_ngs.1) ||
                       (val == 'v' && current_neigh.0 > tmp_ngs.0)
                        {
                        break 'outer;
                    }
                }
                match intersections.get(tmp_ngs) {
                    Some(_) => {closest_neigh.push((tmp_ngs.0, tmp_ngs.1, distance + 1));
                                break 'outer},
                    None => {current_neigh = tmp_ngs.clone();
                            break}
                }
            }
            distance += 1;
        }
    }
    intersections.insert(point.clone(), closest_neigh);    
}

fn find_neighbors(point: &(i32, i32), board: &Vec<Vec<char>>, is_slippery: bool) -> Vec<(i32, i32)>{
    let (max_rows, max_cols): (i32, i32) = (board.len().try_into().unwrap(),
                                            board[0].len().try_into().unwrap());
    let current_value = board[point.0 as usize][point.1 as usize];
    let mut neighs: Vec<(i32, i32)> = vec![];
    if is_slippery{
        if current_value == '>'{
            neighs.push((point.0, point.1 + 1));
            return neighs 
        }
        if current_value == 'v'{
            neighs.push((point.0 + 1, point.1));
            return neighs 
        }
    }

    let row_diff: [i32; 4] = [1, 0, -1, 0];
    let col_diff: [i32; 4] = [0, 1, 0, -1];

    let mut tmp_point: (i32, i32);
    for (delta_x, delta_y) in row_diff.iter().zip(col_diff){
        tmp_point = (point.0 + delta_x, point.1 + delta_y);
        if &tmp_point.0 >= &0 && &tmp_point.0 < &max_rows && &tmp_point.1 >= &0 && &tmp_point.1 < &max_cols{
            let value = board[tmp_point.0 as usize][tmp_point.1 as usize];
            if is_slippery{
                if (value == '>' && delta_x == &-1) || (value == 'v' && delta_y == -1){
                    continue;
                }
            }
            if value != '#'{
                neighs.push(tmp_point)
            }
        }
    }
    return neighs;

}
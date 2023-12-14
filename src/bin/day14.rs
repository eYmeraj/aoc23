#[path = "../utils/utils.rs"]
mod utils;
fn main() {
    use std::time::Instant;
    let mut now = Instant::now();
    let file_path: &str = "src/../data/input14.txt";
    let content: String = utils::read_file(file_path);
    let data: Vec<String> = content
        .split("\n")
        .map(|s| {
            s.to_string()
                .replace("O", "1")
                .replace("#", "2")
                .replace(".", "0")
        })
        .collect();
    part_1(&data);
    println!("Part 1 time: {:.3?}", now.elapsed());
    now = Instant::now();
    part_2(&data);
    println!("Part 2 time: {:.3?}", now.elapsed());
}

fn part_1(data: &Vec<String>) {
    let mut tmp_grid: Vec<Vec<u32>> = vec![];
    for row in data.iter() {
        let tmp_row: Vec<u32>;
        tmp_row = row.chars().map(|i| i.to_digit(10).unwrap()).collect();
        tmp_grid.push(tmp_row);
    }
    let mut grid = rotate_grid(tmp_grid);
    for row_idx in 0..grid.len() {
        let mut tmp_row = grid[row_idx].clone();
        loop {
            let inner_tmp_row = tmp_row.clone();
            let sub_range = (1..grid[0].len()).rev();
            for col_idx in sub_range {
                if tmp_row[col_idx] == 1 {
                    let mut new_idx = grid[row_idx].len();
                    let search_range = (0..col_idx).rev();
                    for search_idx in search_range {
                        if tmp_row[search_idx] == 0 {
                            new_idx = search_idx;
                        } else {
                            break;
                        }
                    }
                    if new_idx != grid[row_idx].len() {
                        tmp_row[new_idx] = 1;
                        tmp_row[col_idx] = 0;

                    }
                }
            }
            if tmp_row == inner_tmp_row {
                break;
            }
        }
        grid[row_idx] = tmp_row;
    }
    let mut tot: usize = 0;
    let max_r = grid[0].len();
    for i in 0..grid.len(){
        for j in 0..max_r{
            if grid[i][j] == 1{
                tot += max_r - j;
            }
        }
    }
    println!("{}", tot);
}

fn part_2(data: &Vec<String>) {
    let n: usize = 1000000000;
    let mut tmp_grid: Vec<Vec<u32>> = vec![];
    for row in data.iter() {
        let tmp_row: Vec<u32>;
        tmp_row = row.chars().map(|i| i.to_digit(10).unwrap()).collect();
        tmp_grid.push(tmp_row);
    }
    let mut grid = rotate_grid(tmp_grid);
    let mut save_cycle: Vec<String> = vec![];
    let mut end_loop = false;
    save_cycle.push(grid_to_str(&grid));

    for _ in 0..n{
        for _ in 0..4{
            for row_idx in 0..grid.len() {
                let mut tmp_row = grid[row_idx].clone();
                loop {
                    let inner_tmp_row = tmp_row.clone();
                    let sub_range = 0..grid[0].len() - 1;
                    for col_idx in sub_range {
                        if tmp_row[col_idx] == 1 {
                            let mut new_idx = grid[row_idx].len();
                            let search_range = col_idx + 1..grid[0].len();
                            for search_idx in search_range {
                                if tmp_row[search_idx] == 0 {
                                    new_idx = search_idx;
                                } else {
                                    break;
                                }
                            }
                            if new_idx != grid[row_idx].len() {
                                tmp_row[new_idx] = 1;
                                tmp_row[col_idx] = 0;
        
                            }
                        }
                    }
                    if tmp_row == inner_tmp_row {
                        break;
                    }
                }
                grid[row_idx] = tmp_row;
            }            
            grid = rotate_grid(grid);
        }
        if end_loop{break}

        let encoded_grid = grid_to_str(&grid);

        if save_cycle.contains(&encoded_grid){
            let index = save_cycle.iter().position(|r| r == &encoded_grid).unwrap();
            if index == save_cycle.len() - 1{
                end_loop = true;
            }
        }else{
            save_cycle.push(encoded_grid);
        }
    }
    let encoded_grid = grid_to_str(&grid);
    let cycle_index_start = save_cycle.iter().position(|r| r == &encoded_grid).unwrap();
    let cycle_length = save_cycle.len() - cycle_index_start;
    let cycle_index_end = cycle_index_start + (n - cycle_index_start) % cycle_length;
    let encoded_end_grid = &save_cycle[cycle_index_end];
    grid = grid_from_str(&encoded_end_grid.split("\n").map(|s| s.to_string()).collect::<Vec<String>>());
    let mut tot: usize = 0;
    let max_r = grid[0].len();
    for i in 0..grid.len(){
        for j in 0..max_r{
            if grid[i][j] == 1{
                tot += j + 1;
            }
        }
    }
    println!("{}", tot);

}

fn grid_to_str(grid: &Vec<Vec<u32>>) -> String{
    let mut grid_of_str: Vec<String> = vec![];
    for row in grid{
        let tmp_string:String = row.iter().map(|i| i.to_string()).collect::<Vec<String>>().join("");
        grid_of_str.push(tmp_string);
    }
    return grid_of_str.join("\n");
}

fn grid_from_str(encoded_grid: &Vec<String>) -> Vec<Vec<u32>>{
    let mut tmp_grid: Vec<Vec<u32>> = vec![];
    for row in encoded_grid.iter() {
        let tmp_row: Vec<u32>;
        tmp_row = row.chars().map(|i| i.to_digit(10).unwrap()).collect();
        tmp_grid.push(tmp_row);
    }
    return tmp_grid;
}


fn rotate_grid(grid: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut rotated_grid: Vec<Vec<u32>> = vec![];
    for j in 0..grid[0].len() {
        let mut tmp_rotated_row: Vec<u32> = vec![];
        for i in (0..grid.len()).rev() {
            tmp_rotated_row.push(grid[i][j]);
        }
        rotated_grid.push(tmp_rotated_row);
    }
    return rotated_grid;
}

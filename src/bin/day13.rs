#[path = "../utils/utils.rs"] mod utils;
fn main() {
    use std::time::Instant;
    let mut now = Instant::now();
    let file_path: &str = "src/../data/input13.txt";
    let content: String = utils::read_file(file_path);
    let data: Vec<String> = content.split("\n\n").map(|s| s.to_string()).collect();
    part_1(&data);
    println!("Part 1 time: {:.3?}", now.elapsed());
    now = Instant::now();
    part_2(&data);
    println!("Part 2 time: {:.3?}", now.elapsed());
}

fn part_1(data: &Vec<String>) {
    let mut tot = 0;
    for map in data.iter(){
        let mut grid: Vec<Vec<char>> = vec![];
        let tmp_data: Vec<String> = map.split("\n").map(|s| s.to_string()).collect();
        for row in tmp_data{
            let mut tmp_vec: Vec<char> = vec![];
            for val in row.chars(){
                tmp_vec.push(val);
            }
            grid.push(tmp_vec);
        }
        let mut grid_t: Vec<Vec<char>> = vec![];
        for j in 0..grid[0].len(){
            let mut tmp_vec: Vec<char> = vec![];
            for i in 0..grid.len(){
                tmp_vec.push(grid[i][j]);
            }
            grid_t.push(tmp_vec);
        }

        tot += 100 * mirror_search(&grid, 0);
        tot += 1 * mirror_search(&grid_t, 0);

    }

    println!("{:?}", tot)
}

fn part_2(data: &Vec<String>) {
    let mut tot = 0;
    for map in data.iter(){
        let mut grid: Vec<Vec<char>> = vec![];
        let tmp_data: Vec<String> = map.split("\n").map(|s| s.to_string()).collect();
        for row in tmp_data{
            let mut tmp_vec: Vec<char> = vec![];
            for val in row.chars(){
                tmp_vec.push(val);
            }
            grid.push(tmp_vec);
        }
        let mut grid_t: Vec<Vec<char>> = vec![];
        for j in 0..grid[0].len(){
            let mut tmp_vec: Vec<char> = vec![];
            for i in 0..grid.len(){
                tmp_vec.push(grid[i][j]);
            }
            grid_t.push(tmp_vec);
        }

        tot += 100 * mirror_search(&grid, 1);
        tot += 1 * mirror_search(&grid_t, 1);

    }

    println!("{:?}", tot)
}

fn mirror_search(grid: &Vec<Vec<char>>, smudges: i32) -> i32{
    let rows = grid.len();
    for line_idx in 1..rows{
        let (up_range, down_range): (std::ops::Range<usize>, std::ops::Range<usize>);
        if line_idx <= rows-line_idx{
            up_range = 0..line_idx;
            down_range = line_idx..line_idx+line_idx;
        }else{
            up_range = line_idx - (rows-line_idx)..line_idx;
            down_range = line_idx..rows;
        }
        let max_matches: i32 = <usize as TryInto<i32>>::try_into(up_range.len()).unwrap() * &grid[0].len().try_into().unwrap();
        let mut matches = max_matches;
        for (row_up, row_down) in up_range.zip(down_range.rev()){
            let tmp_row_up = &grid[row_up];
            let tmp_row_down = &grid[row_down];
            for (val_up, val_down) in tmp_row_up.iter().zip(tmp_row_down){
                if val_up != val_down{
                    matches -= 1;
                }    
            }
        }
        if matches == max_matches - smudges {
            return line_idx as i32;
        }
    }
    return 0;
}



use std::collections::{HashMap, BinaryHeap};

#[path = "../utils/utils.rs"] mod utils;

fn main() {
    use std::time::Instant;
    let mut now = Instant::now();
    let file_path: &str = "src/../data/input17.txt";
    let content: String = utils::read_file(file_path);
    let data: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
    let mut board: Vec<Vec<i32>> = vec![];
    for row in data.iter(){
        let mut tmp_vec: Vec<i32> = vec![];
        for num in row.chars(){
            tmp_vec.push(num.to_digit(10).unwrap() as i32)
        }
        board.push(tmp_vec);
    }
    part_1(&board);
    println!("Part 1 time: {:.3?}", now.elapsed());
    now = Instant::now();
    part_2(&board);
    println!("Part 2 time: {:.3?}", now.elapsed());
}

fn part_1(board: &Vec<Vec<i32>>){

    // the heap data strucure has (distance to source, row, col, (direction row, direction col), steps)
    // all negative since BinaryHeap is max heap
    let mut heap: BinaryHeap<(i32, i32, i32, (i32, i32), i32)> = BinaryHeap::new();

    let right: (i32, i32) = (0, 1);
    let left: (i32, i32) = (0, -1);
    let down: (i32, i32) = (1, 0);
    let up: (i32, i32) = (-1, 0);

    let potential_directions: [(i32, i32); 4] = [right, left, down, up];

    let max_rows: i32 = board.len().try_into().unwrap();
    let max_cols: i32 = board[0].len().try_into().unwrap();
    let source: (i32, i32) = (0, 0);
    let target: (i32, i32) = (max_rows - 1, max_cols -1);
    let mut seen: HashMap<(i32, i32, (i32, i32), i32), bool> = HashMap::new();
    let min_steps = 1;
    let max_steps = 3;
    heap.push((0, - source.0, - source.1, right, 1));
    heap.push((0, - source.0, - source.1, down, 1));

    while heap.len() > 0 {
        let (mut distance, mut row, mut col, direction, steps) = heap.pop().unwrap();
        distance = -distance;
        row = -row;
        col = -col;

        let is_seen = seen.get(&(row, col, direction, steps));
        if is_seen == None{
            seen.insert((row, col, direction, steps), true);
        } else{
            continue;
        }

        let new_row = row + direction.0;
        let new_col = col + direction.1;

        if new_row < 0 || new_col < 0 || new_row >= max_rows || new_col >= max_cols{
            continue;
        }
        let new_distance = distance + board[new_row as usize][new_col as usize];
        if min_steps <= steps && steps <= max_steps{
            if (new_row, new_col) == target{
                println!("{}", new_distance);
                break;
            }
        }
        for heading in potential_directions{
            if (heading.0 + direction.0) == 0 && (heading.1 + direction.1) == 0{
                continue;
            }
            let new_steps: i32;
            if heading == direction {
                new_steps = steps + 1;
            }else{
                new_steps = 1;
            }

            if (direction != heading && steps < min_steps) || new_steps > max_steps{
                continue;
            }

            heap.push((-new_distance, -new_row, -new_col, heading, new_steps));
        }
    }

}


fn part_2(board: &Vec<Vec<i32>>){
        // the heap data strucure has (distance to source, row, col, (direction row, direction col), steps)
    // all negative since BinaryHeap is max heap
    let mut heap: BinaryHeap<(i32, i32, i32, (i32, i32), i32)> = BinaryHeap::new();

    let right: (i32, i32) = (0, 1);
    let left: (i32, i32) = (0, -1);
    let down: (i32, i32) = (1, 0);
    let up: (i32, i32) = (-1, 0);

    let potential_directions: [(i32, i32); 4] = [right, left, down, up];

    let max_rows: i32 = board.len().try_into().unwrap();
    let max_cols: i32 = board[0].len().try_into().unwrap();
    let source: (i32, i32) = (0, 0);
    let target: (i32, i32) = (max_rows - 1, max_cols -1);
    let mut seen: HashMap<(i32, i32, (i32, i32), i32), bool> = HashMap::new();
    let min_steps = 4;
    let max_steps = 10;
    heap.push((0, - source.0, - source.1, right, 1));
    heap.push((0, - source.0, - source.1, down, 1));

    while heap.len() > 0 {
        let (mut distance, mut row, mut col, direction, steps) = heap.pop().unwrap();
        distance = -distance;
        row = -row;
        col = -col;

        let is_seen = seen.get(&(row, col, direction, steps));
        if is_seen == None{
            seen.insert((row, col, direction, steps), true);
        } else{
            continue;
        }

        let new_row = row + direction.0;
        let new_col = col + direction.1;

        if new_row < 0 || new_col < 0 || new_row >= max_rows || new_col >= max_cols{
            continue;
        }
        let new_distance = distance + board[new_row as usize][new_col as usize];
        if min_steps <= steps && steps <= max_steps{
            if (new_row, new_col) == target{
                println!("{}", new_distance);
                break;
            }
        }
        for heading in potential_directions{
            if (heading.0 + direction.0) == 0 && (heading.1 + direction.1) == 0{
                continue;
            }
            let new_steps: i32;
            if heading == direction {
                new_steps = steps + 1;
            }else{
                new_steps = 1;
            }

            if (direction != heading && steps < min_steps) || new_steps > max_steps{
                continue;
            }

            heap.push((-new_distance, -new_row, -new_col, heading, new_steps));
        }
    }

}
use std::collections::HashMap;

#[path = "../utils/utils.rs"] mod utils;

fn main() {
    use std::time::Instant;
    let mut now = Instant::now();
    let file_path: &str = "src/../data/input22.txt";
    let content: String = utils::read_file(file_path);
    let data: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
    let bricks: Vec<[[i32; 3]; 2]> = parse_input(data);
    part_1(bricks.clone());
    println!("Part 1 time: {:.3?}", now.elapsed());
    now = Instant::now();
    part_2(bricks.clone());
    println!("Part 2 time: {:.3?}", now.elapsed());
    
}

fn part_1(mut bricks: Vec<[[i32; 3]; 2]>){
    let mut counter = 0;
    let mut brick_supports_x: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut brick_is_supported_x: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut first_brick = bricks.pop().unwrap();
    while first_brick[0][0] != 1{
        first_brick = lower_brick(first_brick);
    }
    let mut processed_bricks: Vec<[[i32; 3]; 2]> = vec![];
    processed_bricks.push(first_brick);

    while bricks.len() > 0{
        counter += 1;
        let mut current_brick = bricks.pop().unwrap();
        let mut finished: bool = false;
        'outer: loop {
            if current_brick[0][0] == 1{
                processed_bricks.push(current_brick);
                break 'outer;
            }
            for (idx, brick) in processed_bricks.iter().enumerate(){
                if a_supports_b(&brick, &current_brick){
                    println!("{:?}, {:?}, {}, {}", &brick, &current_brick, idx, counter);
                    if !brick_supports_x.contains_key(&(idx as i32)){
                        brick_supports_x.insert(idx as i32, vec![]);
                    }
                    let mut current_list_supported: Vec<i32> = brick_supports_x.get(&(idx as i32)).unwrap().clone();
                    current_list_supported.push(counter);
                    brick_supports_x.insert(idx as i32, current_list_supported);
                    finished = true;
                }
            }
            if finished{
                processed_bricks.push(current_brick);
                break 'outer;
            }
            current_brick = lower_brick(current_brick);
        }
        if counter > 100{
            break;
        }
    } 

    for (key, val) in brick_supports_x.iter(){
        for brk in val.iter(){
            if !brick_is_supported_x.contains_key(brk){
                brick_is_supported_x.insert(brk.clone(), vec![]);
            }
            let mut current_list_supports: Vec<i32> = brick_is_supported_x.get(brk).unwrap().clone();
            current_list_supports.push(key.clone());
            brick_is_supported_x.insert(brk.clone(), current_list_supports);
        }
    }

    let mut tot = 0;
    for (_, supported_bricks) in brick_supports_x.iter(){

        let mut can_destroy: i32 = supported_bricks.len().try_into().unwrap();
        for brk in supported_bricks.iter(){
            let is_supp_by = brick_is_supported_x.get(brk).unwrap();
            if is_supp_by.len() > 1{
                can_destroy -= 1;
            }
        }
        if can_destroy == 0{
            tot += 1;
        }
    }

    for (brick, _) in brick_is_supported_x.iter(){
        if !brick_supports_x.contains_key(brick){
            tot += 1
        }
    }
    println!("{}", tot)
    
}

fn part_2(_bricks: Vec<[[i32; 3]; 2]>){
}


fn parse_input(data: Vec<String>) -> Vec<[[i32; 3]; 2]>{
    let mut bricks: Vec<[[i32; 3]; 2]> = vec![];
    for row in data.iter(){
        let brick_info: Vec<String> = row.split("~").map(|s| s.to_string()).collect();
        let brick_start_str: Vec<String> = brick_info[0].split(",").map(|s| s.to_string()).collect();
        let brick_end_str: Vec<String> = brick_info[1].split(",").map(|s| s.to_string()).collect();

        let brick_start: [i32; 3] = [convert_to_int(brick_start_str[2].clone()), convert_to_int(brick_start_str[0].clone()), convert_to_int(brick_start_str[1].clone())];
        let brick_end: [i32; 3] = [convert_to_int(brick_end_str[2].clone()), convert_to_int(brick_end_str[0].clone()), convert_to_int(brick_end_str[1].clone())];

        bricks.push([brick_start, brick_end])
    }
    bricks.sort();
    bricks.reverse();
   return bricks; 
}

fn convert_to_int(num: String) -> i32{
    return num.parse::<i32>().unwrap();
}

fn lower_brick(brick: [[i32; 3]; 2]) -> [[i32; 3]; 2]{
    let mut brick_down: [[i32; 3]; 2];
    brick_down = brick.clone();
    brick_down[0][0] -= 1;
    brick_down[1][0] -= 1;
    return brick_down;
}

fn a_supports_b(brick_a: &[[i32; 3]; 2], brick_b: &[[i32; 3]; 2]) -> bool{
    // a is meant to be the lower brick.
    if brick_a[0][0] + 1 == brick_b[0][0] {
        let (rest_range_x1, rest_range_x2) = (brick_a[0][1], brick_a[1][1]);
        let (rest_range_y1, rest_range_y2) = (brick_a[0][2], brick_a[1][2]);
        let (floaring_range_x1, floaring_range_x2) = (brick_b[0][1], brick_b[1][1]);
        let (floaring_range_y1, floaring_range_y2) = (brick_b[0][2], brick_b[1][2]);

        if (rest_range_x1 <= floaring_range_x2 && floaring_range_x1 <= rest_range_x2) && (rest_range_y1 <= floaring_range_y2 && floaring_range_y1 <= rest_range_y2){
            return true;
        }
        return false
    }else{
        return false;
    }
}
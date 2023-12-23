use std::collections::{HashMap, VecDeque};
use std::time::Instant;

#[path = "../utils/utils.rs"] mod utils;

fn main() {
    let mut now = Instant::now();
    let file_path: &str = "src/../data/input20.txt";
    let content: String = utils::read_file(file_path);
    let mut data: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
    let starting_modules: Vec<String>;
    let mut starting_idx: usize = 0;
    
    for (idx, row) in data.iter().enumerate(){
        if row.contains("broadcaster"){
            starting_idx = idx;
            break
        }
    }
    let tmp_starting_row = data.remove(starting_idx);
    let binding = tmp_starting_row.split(" -> ").map(|s| s.to_string()).collect::<Vec<String>>();
    starting_modules = binding[1].split(", ").map(|s| s.to_string()).collect::<Vec<String>>().to_vec();
    
    let mut flip_flops: HashMap<String, u32> = HashMap::new();
    let mut conjucntions: HashMap<String, (Vec<String>, Vec<u32>)> = HashMap::new();
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    
    
    for row in data.iter(){
        let input: String;
        input = row.split(" -> ").map(|s| s.to_string()).collect::<Vec<String>>()[0].clone();

        if input.contains("%") {
            flip_flops.insert(input.replace("%", ""), 0);
        }else{
            conjucntions.insert(input.replace("&", ""), (vec![], vec![]));
        }
    }

    for row in data.iter(){
        let input: String;
        let target_string: String;
        let target: Vec<String>;
        input = row.split(" -> ").map(|s| s.to_string()).collect::<Vec<String>>()[0].clone().replace("&", "").replace("%", "");
        target_string = row.split(" -> ").map(|s| s.to_string()).collect::<Vec<String>>()[1].clone();
        target = target_string.split(", ").map(|s| s.to_string()).collect::<Vec<String>>();

        for conn in target.iter(){
            if conjucntions.contains_key(conn){
                let mut current_value: (Vec<String>, Vec<u32>) = conjucntions.get(conn).unwrap().clone();
                current_value.0.push(input.clone());
                current_value.1.push(0);
                conjucntions.insert(conn.to_string(), current_value);
            }
        }
    }

    for row in data.iter(){
        let input: String;
        let target_string: String;
        let target: Vec<String>;

        input = row.split(" -> ").map(|s| s.to_string()).collect::<Vec<String>>()[0].clone().replace("&", "").replace("%", "");
        target_string = row.split(" -> ").map(|s| s.to_string()).collect::<Vec<String>>()[1].clone();
        target = target_string.split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
        rules.insert(input, target);
    }

    part_1(&starting_modules, &rules, flip_flops.clone(), conjucntions.clone());
    println!("Part 1 time: {:.3?}", now.elapsed());
    now = Instant::now();
    part_2(&starting_modules, &rules, flip_flops.clone(), conjucntions.clone());
    println!("Part 2 time: {:.3?}", now.elapsed());
    
}

fn part_1(starting_modules: &Vec<String>, rules: &HashMap<String, Vec<String>>, mut flip_flops: HashMap<String, u32>, mut conjunctions: HashMap<String, (Vec<String>, Vec<u32>)>){
    let mut pulses: HashMap<String, u32> = HashMap::new();
    pulses.insert("low".to_string(), 0);
    pulses.insert("high".to_string(), 0);
    let iterations: u32 = 1000;
    for _ in 0..iterations{
        match pulses.get(&"low".to_string()) {
            Some(count) => {pulses.insert("low".to_string(), count + 1);}
            None => {}
        }
        let mut queue: VecDeque<(String, String, String)> = VecDeque::new();
        for module in starting_modules{
            queue.push_back((module.clone(), "low".to_string(), "button".to_string()));
        }

        while queue.len() > 0{
            let current_action = queue.pop_front().unwrap();
            let new_actions = hande_pulse(current_action, rules, &mut flip_flops, &mut conjunctions, &mut pulses);
            for action in new_actions.iter(){
                queue.push_back(action.clone());
            }
        }
    }
    let lows = pulses.get("low").unwrap();
    let highs = pulses.get("high").unwrap();
    println!("{}", lows * highs);
}

fn part_2(starting_modules: &Vec<String>, rules: &HashMap<String, Vec<String>>, mut flip_flops: HashMap<String, u32>, mut conjunctions: HashMap<String, (Vec<String>, Vec<u32>)>){
    let mut pulses: HashMap<String, u32> = HashMap::new();
    pulses.insert("low".to_string(), 0);
    pulses.insert("high".to_string(), 0);

    let mut module_to_rx: String = String::from("_");
    for (k, v) in rules.iter(){
        if v.contains(&"rx".to_string()){
            module_to_rx = k.clone();
            break;
        }
    }
    let modules_need_high: Vec<String>;
    modules_need_high = conjunctions.get(&module_to_rx).unwrap().0.clone();
    let mut cycles: Vec<u32> = vec![];
    for _ in 0..modules_need_high.len(){
        cycles.push(0);
    }

    let iterations: u32 = 10000;
    for i in 1..iterations{
        match pulses.get(&"low".to_string()) {
            Some(count) => {pulses.insert("low".to_string(), count + 1);}
            None => {}
        }
        let mut queue: VecDeque<(String, String, String)> = VecDeque::new();
        for module in starting_modules{
            queue.push_back((module.clone(), "low".to_string(), "button".to_string()));
        }

        while queue.len() > 0{
            let current_status = conjunctions.get(&module_to_rx).unwrap().1.clone();
            for k in 0..modules_need_high.len(){
                if current_status[k] != 0 && cycles[k] == 0{
                    cycles[k] = i;
                }
            }
            let current_action = queue.pop_front().unwrap();
            let new_actions = hande_pulse(current_action, rules, &mut flip_flops, &mut conjunctions, &mut pulses);
            for action in new_actions.iter(){
                queue.push_back(action.clone());
            }
        }

        if !cycles.contains(&0){
            break
        }
    }
    let mut tot: usize = 1;
    for num in cycles{
        tot = utils::lcm(tot, num as usize);
    }
    println!("{}", tot);
}

fn hande_pulse(action: (String, String, String), rules: &HashMap<String, Vec<String>>, flip_flops: &mut HashMap<String, u32>, conjunctions: &mut HashMap<String, (Vec<String>, Vec<u32>)>, pulses: &mut HashMap<String, u32>) -> Vec<(String, String, String)>{
    let (module, signal_type, sender) = action;
    let current_pulses = pulses.get(&signal_type).unwrap();
    pulses.insert(signal_type.clone(), current_pulses + 1);
    let targets: &Vec<String>;
    let mut next_actions: Vec<(String, String, String)>;
    let tmp_targtes = &vec![];
    match rules.get(&module) {
        Some(values) => {targets = values;}
        None => {targets = tmp_targtes;}
    }
    if flip_flops.contains_key(&module){
        if signal_type.contains("high"){
            next_actions = vec![];
            return next_actions
        }else{
            if flip_flops.get(&module).unwrap() == &0{
                flip_flops.insert(module.clone(), 1);
                next_actions = vec![];
                for next_module in targets.iter(){
                    next_actions.push((next_module.clone(), "high".to_string(), module.clone()));
                }
                return next_actions;
            }else{
                flip_flops.insert(module.clone(), 0);
                next_actions = vec![];
                for next_module in targets.iter(){
                    next_actions.push((next_module.clone(), "low".to_string(), module.clone()));
                }
                return next_actions;
            }
        }
    }else if conjunctions.contains_key(&module){
        let mut current_state = conjunctions.get(&module).unwrap().clone();
        let idx = current_state.0.iter().position(|x| *x == sender).unwrap();
        next_actions = vec![];
        if signal_type == "high".to_string(){
            current_state.1[idx] = 1
        }else{
            current_state.1[idx] = 0
        }
        if current_state.1.contains(&0){
            for next_module in targets.iter(){
                next_actions.push((next_module.clone(), "high".to_string(), module.clone()));
            }
        }else{
            for next_module in targets.iter(){
                next_actions.push((next_module.clone(), "low".to_string(), module.clone()));
            }
        }
        conjunctions.insert(module, current_state.clone());
        return next_actions;
    }else{
        return vec![];
    }
}


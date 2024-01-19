#[path = "../utils/utils.rs"] mod utils;
extern crate nalgebra as na;
use na::{DMatrix,  DVector};

fn main() {
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=framework=Accelerate");
    use std::time::Instant;
    let mut now = Instant::now();
    let file_path: &str = "src/../data/input24.txt";
    let content: String = utils::read_file(file_path);
    let data: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
    let (coordinates, velocities): (Vec<(i64, i64, i64)>, Vec<(i64, i64, i64)>) = parse_input(data);
    part_1(&coordinates, &velocities);
    println!("Part 1 time: {:.3?}", now.elapsed());
    now = Instant::now();
    part_2(&coordinates, &velocities);
    println!("Part 2 time: {:.3?}", now.elapsed());
}

fn part_1(coordinates: &Vec<(i64, i64, i64)>, velocities: &Vec<(i64, i64, i64)>){
    let (min_coord, max_coord) : (i64, i64) = (200000000000000, 400000000000000);
    let mut tot: i64 = 0;
    for i in 0..coordinates.len(){
        for j in (i+1)..coordinates.len(){
            let (p1, v1) = (coordinates[i], velocities[i]);
            let (p2, v2) = (coordinates[j], velocities[j]);
            if intersection(&p1, &v1, &p2, &v2, min_coord, max_coord){
                tot += 1;
            }

        }
    }
    println!("{}", tot);
}


fn part_2(coordinates: &Vec<(i64, i64, i64)>, velocities: &Vec<(i64, i64, i64)>){
    find_solution_eq_system(coordinates, velocities);
}

fn parse_input(data: Vec<String>) -> (Vec<(i64, i64, i64)>, Vec<(i64, i64, i64)>){
    let (mut coordinates, mut velocities): (Vec<(i64, i64, i64)>, Vec<(i64, i64, i64)>) = (vec![], vec![]);
    for mut row in data{
        row = row.replace(" ", "");
        let tmp_coords_str: String;
        let tmp_vs_str: String;
        let hold: Vec<String> = row.split("@").map(|s| s.to_string()).collect();
        tmp_coords_str = hold[0].clone();
        tmp_vs_str = hold[1].clone();
        let tmp_coords: Vec<i64>= tmp_coords_str.split(",").map(|s| s.parse::<i64>().unwrap()).collect(); 
        let tmp_vs: Vec<i64> = tmp_vs_str.split(",").map(|s| s.parse::<i64>().unwrap()).collect();
        coordinates.push((tmp_coords[0], tmp_coords[1], tmp_coords[2]));
        velocities.push((tmp_vs[0], tmp_vs[1], tmp_vs[2]));
    }
    return (coordinates, velocities);
}

fn find_line(p1: &(i64, i64, i64), velocity: &(i64, i64, i64)) -> (f64, f64){
    let p2: (i64, i64) = (p1.0 + velocity.0, p1.1 + velocity.1);
    let m : f64= (p2.1 as f64 - p1.1 as f64)/(p2.0 as f64 - p1.0 as f64);
    let c : f64= p1.1 as f64- m*(p1.0 as f64);
    return (m, c)
}

fn intersection(p1: &(i64, i64, i64), v1: &(i64, i64, i64), 
                p2: &(i64, i64, i64), v2: &(i64, i64, i64),
                min_coord: i64, max_coord: i64) -> bool{
    let (m1, c1) = find_line(p1, v1);
    let (m2, c2) = find_line(p2, v2);
    if (m1 - m2).abs() < 1.0/1e6_f64 {
        return false;
    }
    let xo: (f64, f64) = ((c1- c2)/(m2-m1), (c1*m2 - c2*m1)/(m2-m1));
    let time_p1_xo = (xo.0 - p1.0 as f64)/(v1.0 as f64);
    let time_p2_xo = (xo.0 - p2.0 as f64)/(v2.0 as f64);
    if time_p1_xo < 0.0 || time_p2_xo < 0.0 {
        return false;
    }
    if min_coord as f64 <= xo.0 && xo.0 <= max_coord as f64 && min_coord as f64 <= xo.1 && xo.1 <= max_coord as f64{
        return true;
    }

    return false;
}


fn find_solution_eq_system(coordinates: &Vec<(i64, i64, i64)>, velocities: &Vec<(i64, i64, i64)>){
    let (x0, y0, z0): (f64, f64, f64) = (coordinates[0].0 as f64, coordinates[0].1 as f64, coordinates[0].2 as f64);
    let (vx0, vy0, vz0): (f64, f64, f64) = (velocities[0].0 as f64, velocities[0].1 as f64, velocities[0].2 as f64);
    let (x1, y1, z1): (f64, f64, f64) = (coordinates[1].0 as f64, coordinates[1].1 as f64, coordinates[1].2 as f64);
    let (vx1, vy1, vz1): (f64, f64, f64) = (velocities[1].0 as f64, velocities[1].1 as f64, velocities[1].2 as f64);
    let (x2, y2, z2): (f64, f64, f64) = (coordinates[2].0 as f64, coordinates[2].1 as f64, coordinates[2].2 as f64);
    let (vx2, vy2, vz2): (f64, f64, f64) = (velocities[2].0 as f64, velocities[2].1 as f64, velocities[2].2 as f64);
    let mut current: DVector<f64> = DVector::from_row_slice(&[100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0]);

    let tol: f64 = 1.0/1e5_f64;
    loop {
        let (fx,jx) = f_x_j_x(&current, x0, y0, z0, vx0, vy0, vz0, x1, y1, z1, vx1, vy1, vz1, x2, y2, z2, vx2, vy2, vz2);
        println!("{:?}", jx.transpose());
        let jx_inv = jx.try_inverse().unwrap();
        let y_new_arr = jx_inv * fx;
        let x_new = current.clone() + y_new_arr;
        let norm = norm_2(&x_new, &current);
        if norm < tol{
            println!("{:?}", x_new);
            break;
        }else{
            current = x_new;
        }
    }
}


fn norm_2(x_k1: &DVector<f64>,
          x_k: &DVector<f64>)
          -> f64{
    let mut norm_sum: f64 = 0.0;
    norm_sum += (x_k1[0] - x_k[0]).powi(2);
    norm_sum += (x_k1[1] - x_k[1]).powi(2);
    norm_sum += (x_k1[2] - x_k[2]).powi(2);
    norm_sum += (x_k1[3] - x_k[3]).powi(2);
    norm_sum += (x_k1[4] - x_k[4]).powi(2);
    norm_sum += (x_k1[5] - x_k[5]).powi(2);
    norm_sum += (x_k1[6] - x_k[6]).powi(2);
    norm_sum += (x_k1[7] - x_k[7]).powi(2);
    norm_sum += (x_k1[8] - x_k[8]).powi(2);

    norm_sum = norm_sum.sqrt();
    return norm_sum;
}
fn f_x_j_x(x_vec: &DVector<f64>, 
        x0: f64, y0: f64, z0: f64, vx0: f64, vy0: f64, vz0: f64,
        x1: f64, y1: f64, z1: f64, vx1: f64, vy1: f64, vz1: f64,
        x2: f64, y2: f64, z2: f64, vx2: f64, vy2: f64, vz2: f64)
        -> (DVector<f64>, DMatrix<f64>){
    let (x, y, z, vx, vy, vz, t0, t1, t2) = (x_vec[0], x_vec[1], x_vec[2], x_vec[3], x_vec[4], x_vec[5], x_vec[6], x_vec[7], x_vec[8]);

    let fx: DVector<f64> = DVector::from_row_slice(&[
            x + t0*vx - x0 - t0*vx0,
            x + t1*vx - x1 - t1*vx1,
            x + t2*vx - x2 - t2*vx2,
            y + t0*vy - y0 - t0*vy0,
            y + t1*vy - y1 - t1*vy1,
            y + t2*vy - y2 - t2*vy2,
            z + t0*vz - z0 - t0*vz0,
            z + t1*vz - z1 - t1*vz1,
            z + t2*vz - z2 - t2*vz2]);
        
    let jx: DMatrix<f64> = DMatrix::from_row_slice(9, 9, &[
        1.0, 0.0, 0.0,   t0,  0.0,  0.0, vx-vx0,    0.0,    0.0,
        1.0, 0.0, 0.0,   t1,  0.0,  0.0,    0.0, vx-vx1,    0.0,
        1.0, 0.0, 0.0,   t2,  0.0,  0.0,    0.0,    0.0, vx-vx2,
        0.0, 1.0, 0.0,  0.0,   t0,  0.0, vy-vy0,    0.0,    0.0,
        0.0, 1.0, 0.0,  0.0,   t1,  0.0,    0.0, vy-vy1,    0.0,
        0.0, 1.0, 0.0,  0.0,   t2,  0.0,    0.0,    0.0, vy-vy2,
        0.0, 0.0, 1.0,  0.0,  0.0,   t0, vz-vz0,    0.0,    0.0,
        0.0, 0.0, 1.0,  0.0,  0.0,   t1,    0.0, vz-vz1,    0.0,
        0.0, 0.0, 1.0,  0.0,  0.0,   t2,    0.0,    0.0, vz-vz2]);
    
    return (fx, jx); 
    
}


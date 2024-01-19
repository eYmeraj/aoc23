#[path = "utils/utils.rs"] mod utils;
extern crate nalgebra as na;
use na::{DMatrix,  DVector};
fn main() {
    println!("AOC23!");
    let mut mat = DMatrix::from_row_slice(3, 3, &[
        1.0, 2.0, 3.0,
        7.0, 1.0, 3.0,
        0.0, -2.0, 1.0]);
    
    let vc = DVector::from_row_slice(&[1.0, 2.0, 3.0]);
    let vc2 = DVector::from_row_slice(&[0.0, 2.0, 3.0]);
    
    // mat = mat.try_inverse().unwrap();

    println!("{mat:?}");
    println!("{:?}", vc[0]);
}

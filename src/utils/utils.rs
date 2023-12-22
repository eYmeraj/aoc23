use std::fs;

#[allow(unused)]
pub fn read_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect(file_path);
    return contents
}

#[allow(unused)]
pub fn irregular_polygon_area_i32(vertecies: &Vec<(i32, i32)>) -> i32{
    /// Uses Shoelace theorem
    /// given a set of verices, with vertices[0] == vertices[-1]
    /// it computes the area of an irregular polygon
    /// formula: 1/2|SUM[(y_i + y_(i+1) * (x_i - x_(i+1)) for i in 0..n-1]|
    let mut area: i32 = 0;
    for idx in 0..vertecies.len() - 1{
        let p1: (i32, i32) = vertecies[idx];
        let p2: (i32, i32) = vertecies[idx + 1];

        area += (p1.1 + p2.1)*(p1.0 - p2.0)
    }
    area = area.abs();
    return area / 2;
}

#[allow(unused)]
pub fn irregular_polygon_area_i64(vertecies: &Vec<(i64, i64)>) -> i64{
    let mut area: i64 = 0;
    for idx in 0..vertecies.len() - 1{
        let p1: (i64, i64) = vertecies[idx];
        let p2: (i64, i64) = vertecies[idx + 1];

        area += (p1.1 + p2.1)*(p1.0 - p2.0)
    }
    area = area.abs();
    return area / 2;
}

#[allow(unused)]
pub fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

#[allow(unused)]
pub fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
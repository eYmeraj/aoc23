use std::fs;

#[allow(unused)]
pub fn read_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect(file_path);
    return contents
}

#[allow(unused)]
pub fn irregular_polygon_area_i32(vertecies: &Vec<(i32, i32)>) -> i32{
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
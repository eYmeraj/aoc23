use std::fs;

#[allow(unused)]
pub fn read_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect(file_path);
    return contents
}
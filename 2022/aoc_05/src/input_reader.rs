use std::{env, fs};

pub fn read_file() -> String {
    let args: Vec<String> = env::args().collect();
    let executable_name = args.get(0).unwrap();
    let input_file_path = args.get(1).expect(format!("Usage: {executable_name} <input file path>").as_str());
    return fs::read_to_string(input_file_path).expect("Should have been able to read the file.");
}
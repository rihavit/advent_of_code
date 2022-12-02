use std::{env, fs};
use std::cmp::max;

fn main() {
    let args: Vec<String> = env::args().collect();
    let executable_name = args.get(0).unwrap();
    let input_file_path = args.get(1).expect(format!("Usage: {executable_name} <input file path>").as_str());

    let input = fs::read_to_string(input_file_path).expect("Should have been able to read the file.");
    let elves: Vec<&str> = input.split("\n\r\n").collect();
    let mut result: u32 = 0;
    for elf in elves {
        result = max(elf.split("\n")
                         .map(|s| if s.is_empty() {0} else {s[..s.len()-1].parse::<u32>().unwrap_or(0)})
                         .sum(), result);
    }
    println!("{result}");
}

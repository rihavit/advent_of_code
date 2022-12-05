use std::cmp::max;
use input_reader;

fn main() {
    let input = input_reader::read_file();
    let elves: Vec<&str> = input.split("\n\r\n").collect();
    let mut top_three: Vec<u32> = Vec::new();
    top_three.reserve(3);
    for elf in elves {
        let total_elf = elf.split("\n")
            .map(|s| if s.is_empty() {0} else {s[..s.len()-1].parse::<u32>().unwrap_or(0)})
            .sum();
        if top_three.len() < 3 {
            top_three.push(total_elf);
            continue;
        }
        for i in 0..3 {
            if top_three[i] < total_elf {
                top_three[i] = total_elf;
                break;
            }
        }
    }
    println!("Max elf: {}", max(max(top_three[0], top_three[1]), top_three[2]));
    println!("Total elves: {}", top_three.iter().sum::<u32>());
}

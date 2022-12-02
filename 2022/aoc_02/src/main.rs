use std::collections::HashMap;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let executable_name = args.get(0).unwrap();
    let input_file_path = args.get(1).expect(format!("Usage: {executable_name} <input file path>").as_str());
    let input = fs::read_to_string(input_file_path).expect("Should have been able to read the file.");

    let score_rules = HashMap::from([
        ("A", HashMap::from([
            ("X", 3 + 1),
            ("Y", 6 + 2),
            ("Z", 0 + 3),
        ])),
        ("B", HashMap::from([
            ("X", 0 + 1),
            ("Y", 3 + 2),
            ("Z", 6 + 3),
        ])),
        ("C", HashMap::from([
            ("X", 6 + 1),
            ("Y", 0 + 2),
            ("Z", 3 + 3),
        ])),
    ]);
    let mut score = 0;
    for round in input.split('\n') {
        if round.is_empty() {continue;}
        let [shape_a, shape_b] = <[&str; 2]>::try_from(round.split_whitespace().take(2).collect::<Vec<&str>>()).ok().unwrap();
        score += score_rules.get(shape_a).unwrap().get(shape_b).unwrap();
    }
    println!("{score}");
}
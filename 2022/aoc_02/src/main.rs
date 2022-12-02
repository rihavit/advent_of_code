use std::collections::HashMap;
mod input_reader;

fn main() {
    let input = input_reader::read_file();
    let score_rules_one = HashMap::from([
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
    let score_rules_two = HashMap::from([
        ("A", HashMap::from([
            ("X", 0 + 3),
            ("Y", 3 + 1),
            ("Z", 6 + 2),
        ])),
        ("B", HashMap::from([
            ("X", 0 + 1),
            ("Y", 3 + 2),
            ("Z", 6 + 3),
        ])),
        ("C", HashMap::from([
            ("X", 0 + 2),
            ("Y", 3 + 3),
            ("Z", 6 + 1),
        ])),
    ]);

    println!("Part one: {}", get_score(&input, &score_rules_one));
    println!("Part two: {}", get_score(&input, &score_rules_two));
}

fn get_score(input: &String, score_rules: &HashMap<&str, HashMap<&str, i32>>) -> i32 {
    return input
        .split('\n')
        .filter(|round| !round.is_empty())
        .map(|round| {
            let [shape_a, shape_b] = <[&str; 2]>::try_from(round.split_whitespace().take(2).collect::<Vec<&str>>()).ok().unwrap();
            return score_rules.get(shape_a).unwrap().get(shape_b).unwrap();
        })
        .sum();
}
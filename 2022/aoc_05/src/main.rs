use lazy_static::lazy_static;
use regex::Regex;
use input_reader;

fn main() {
    let input = input_reader::read_file();
    let mut stacks: Vec<Vec<char>> = vec![
        vec!['W', 'B', 'G', 'Z', 'R', 'D', 'C', 'V'],
        vec!['V', 'T', 'S', 'B', 'C', 'F', 'W', 'G'],
        vec!['W', 'N', 'S', 'B', 'C'],
        vec!['P', 'C', 'V', 'J', 'N', 'M', 'G', 'Q'],
        vec!['B', 'H', 'D', 'F', 'L', 'S', 'T'],
        vec!['N', 'M', 'W', 'T', 'V', 'J'],
        vec!['G', 'T', 'S', 'C', 'L', 'F', 'P'],
        vec!['Z', 'D', 'B'],
        vec!['W', 'Z', 'N', 'M'],
    ];
    for stack in &mut stacks {
        stack.reverse();
    }
    for operation in input.split('\n') {
        perform(operation, &mut stacks);
    }
    for stack in stacks {
        print!("{}", *stack.last().unwrap());
    }
    println!();
}

fn perform(operation: &str, ship: &mut Vec<Vec<char>>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    let re: Regex = Regex::new(r"\d+").unwrap();
    let instructions: Vec<usize> = re.captures_iter(operation)
        .map(|m| m[0].parse::<usize>().unwrap()).collect();
    match (instructions.get(0), instructions.get(1), instructions.get(2)) {
        (Some(&how_many), Some(&from), Some(&to)) => move_items(ship, how_many, from, to),
        _ => panic!("Instruction unclear.")
    }
}

fn move_items(ship: &mut Vec<Vec<char>>, how_many: usize, from: usize, to: usize) {
    let from = from - 1;
    let to = to - 1;
    let at = ship.get(from).unwrap().len() - how_many;
    for _ in 0..how_many {
        let item = ship.get_mut(from).unwrap().remove(at);
        ship.get_mut(to).unwrap().push(item);
    }
}
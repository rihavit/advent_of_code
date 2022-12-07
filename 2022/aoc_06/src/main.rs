fn main() {
    let input = input_reader::read_file().chars().collect::<Vec<char>>();
    println!("{}", get_index_of_seq_of_unique(&input, 14).expect("No unique sequence of this length found."));
}

fn get_index_of_seq_of_unique(input: &Vec<char>, unique_len: usize) -> Option<usize> {
    for i in 0..input.len() {
        if input[i..i + unique_len].iter().enumerate()
            .all(|(j, c)| !input[i + j + 1..i + unique_len].contains(c))
        {
            return Some(i + unique_len);
        };
    }
    return None;
}

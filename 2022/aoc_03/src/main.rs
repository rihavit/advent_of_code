mod input_reader;

fn main() {
    let input = input_reader::read_file();
    let result: u16 = input.split('\n').map(|rucksack| {
        let items = rucksack.chars().collect::<Vec<char>>();
        let cmp_a = &items[..items.len()/2];
        let cmp_b = &items[items.len()/2..];
        return match cmp_a.iter().find(|item| cmp_b.contains(item)) {
            Some(x) => item_to_priority(x),
            _ => panic!("No common item found!")
        }
    }).sum();
    println!("{result}");
}

fn item_to_priority(item: &char) -> u16 {
    let ascii_code = *item as u16;
    return if ascii_code > 90 {
        ascii_code - 96
    } else {
        ascii_code - 38
    }
}
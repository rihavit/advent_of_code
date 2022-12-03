mod input_reader;

fn main() {
    let input = input_reader::read_file();
    let sum_of_priorities: u16 = input.split('\n').map(|rucksack| {
        let items = rucksack.chars().collect::<Vec<char>>();
        let cmp_a = &items[..items.len()/2];
        let cmp_b = &items[items.len()/2..];
        return match cmp_a.iter().find(|item| cmp_b.contains(item)) {
            Some(x) => item_to_priority(x),
            _ => panic!("No common item found!")
        }
    }).sum();
    println!("Sum of common items' priorities: {sum_of_priorities}");

    let sum_of_badges: u16 = input.split('\n').collect::<Vec<&str>>().chunks(3).map(|group| -> u16 {
        let rucksacks = group.iter().map(|elf| elf.chars().collect()).collect::<Vec<Vec<char>>>();
        return match &rucksacks[..] {
            [ rucksack_a, rucksack_b, rucksack_c ] => {
                let badge = rucksack_a.iter().find(|item| {
                    rucksack_b.contains(item) && rucksack_c.contains(item)
                }).unwrap();
                item_to_priority(badge)
            },
            _ => panic!("Group did not contain three rucksacks.")
        }
    }).sum();
    println!("Sum of badges' priorities: {sum_of_badges}")
}

fn item_to_priority(item: &char) -> u16 {
    let ascii_code = *item as u16;
    return if ascii_code > 90 {
        ascii_code - 96
    } else {
        ascii_code - 38
    }
}
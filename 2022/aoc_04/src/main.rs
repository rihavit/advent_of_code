mod input_reader;

struct TaskRange {
    from: u8,
    to: u8
}

impl TaskRange {
    fn new(from: u8, to: u8) -> Self {
        Self { from, to }
    }

    fn parse_range(range_string: &str) -> TaskRange {
        match range_string.split('-').collect::<Vec<&str>>()[..] {
            [ from, to ] => {
                TaskRange::new(from.parse::<u8>().unwrap(), to.trim().parse::<u8>().unwrap())
            },
            _ => panic!("Failed parsing range.")
        }
    }

    fn contains(&self, other: &TaskRange) -> bool {
        self.from <= other.from && self.to >= other.to
    }
}

fn main() {
    let input = input_reader::read_file();
    let overlapping_count = input.split('\n').filter(|pair| {
        if pair.is_empty() { return false }
        let ranges = pair.split(',').collect::<Vec<&str>>();
        let range_a = TaskRange::parse_range(ranges.get(0).unwrap());
        let range_b = TaskRange::parse_range(ranges.get(1).unwrap());
        range_a.contains(&range_b) || range_b.contains(&range_a)
    }).count();
    println!("Overlapping count: {overlapping_count}");
}

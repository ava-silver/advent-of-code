use std::fs::read_to_string;

fn parse_messages(input: String) -> Parsed {}

fn part_1(messages: Parsed) -> usize {}

fn part_2(messages: Parsed) -> usize {
    0
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let messages = parse_messages(input.clone());
    println!("Part 1: {}", part_1(messages.clone()));
    println!("Part 2: {}", part_2(messages));
}

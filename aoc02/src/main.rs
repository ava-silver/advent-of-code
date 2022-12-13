use std::fs::read_to_string;

fn part_1(input: &String) -> u64 {
  input
    .lines()
    .map(|line| {
      let parts = line.split(" ").collect::<Vec<&str>>();
      (match (parts[0], parts[1]) {
        // outcome
        ("A", "Z") | ("B", "X") | ("C", "Y") => 0, // lose
        ("A", "X") | ("B", "Y") | ("C", "Z") => 3, // draw
        ("A", "Y") | ("B", "Z") | ("C", "X") => 6, // win
        _ => 0,
      } + match parts[1] {
        // item
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
      })
    })
    .sum()
}

fn part_2(input: &String) -> u64 {
  input
    .lines()
    .map(|line| {
      let parts = line.split(" ").collect::<Vec<&str>>();
      (match parts[1] {
        // outcome
        "X" => 0, // lose
        "Y" => 3, // draw
        "Z" => 6, // win
        _ => 0,
      } + match (parts[0], parts[1]) {
        // item
        ("A", "Y") | ("C", "Z") | ("B", "X") => 1, // rock
        ("B", "Y") | ("A", "Z") | ("C", "X") => 2, // paper
        ("C", "Y") | ("B", "Z") | ("A", "X") => 3, // scissors
        _ => 0,
      })
    })
    .sum()
}

fn main() {
  let input = read_to_string("input.txt").unwrap();
  println!("Part 1: {}", part_1(&input));
  println!("Part 2: {}", part_2(&input));
}

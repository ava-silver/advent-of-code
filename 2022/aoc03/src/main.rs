use itertools::Itertools;
use std::fs::read_to_string;

fn value(c: char) -> u32 {
  match c {
    'a'..='z' => (c as u32) - 96,
    'A'..='Z' => (c as u32) - 38,
    _ => 0,
  }
}

fn part_1(input: &String) -> u32 {
  input
    .lines()
    .map(|backpack| {
      let midpoint = backpack.len() / 2;
      let first: Vec<char> = backpack[..midpoint].chars().collect();
      for c in backpack[midpoint..].chars() {
        if first.contains(&c) {
          return value(c);
        }
      }
      return 0;
    })
    .sum()
}

fn part_2(input: &String) -> u32 {
  input
    .lines()
    .chunks(3)
    .into_iter()
    .map(|lines| {
      let elves: Vec<Vec<char>> = lines.map(|elf| elf.chars().collect()).collect();
      if elves.len() < 3 {
        return 0;
      }
      for &c in &elves[0] {
        if elves[1].contains(&c) && elves[2].contains(&c) {
          return value(c);
        }
      }
      return 0;
    })
    .sum()
}

fn main() {
  let input = read_to_string("input.txt").unwrap();
  println!("Part 1: {}", part_1(&input));
  println!("Part 2: {}", part_2(&input));
}

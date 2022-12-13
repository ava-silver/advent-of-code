use std::{collections::VecDeque, fs::read_to_string};

fn parse_instructions(input: String) -> Vec<Option<i64>> {
  input
    .lines()
    .map(|l| {
      let mut tok = l.split(' ');
      match (tok.next(), tok.next().and_then(|v| v.parse().ok())) {
        (Some("addx"), v) => v,
        _ => None,
      }
    })
    .collect()
}

fn part_1(instructions: &Vec<Option<i64>>) -> i64 {
  let checks = [20, 60, 100, 140, 180, 220];
  let mut queue: VecDeque<Option<i64>> = instructions
    .iter()
    .flat_map(|&i| {
      if i.is_some() {
        vec![None, i]
      } else {
        vec![None]
      }
    })
    .collect();
  let mut x = 1;
  let mut signal = 0;
  for cycle in 1..=220 {
    if checks.contains(&cycle) {
      signal += cycle * x;
    }
    if let Some(Some(v)) = queue.pop_front() {
      x += v;
    }
  }
  signal
}

fn part_2(instructions: &Vec<Option<i64>>) -> String {
  let mut queue: VecDeque<Option<i64>> = instructions
    .iter()
    .flat_map(|i| match *i {
      None => vec![None],
      v => vec![None, v],
    })
    .collect();
  let mut x = 1;
  let mut display = "".to_owned();
  for cycle in 0..240 {
    if cycle % 40 == 0 {
      display.push('\n');
    }
    display.push(if (cycle as i64 % 40).abs_diff(x) <= 1 {
      '#'
    } else {
      ' '
    });
    if let Some(Some(v)) = queue.pop_front() {
      x += v;
    }
  }
  display
}

fn main() {
  let input = read_to_string("input.txt").unwrap();
  let instructions = parse_instructions(input);
  println!("Part 1: {}", part_1(&instructions));
  println!("Part 2: \n{}", part_2(&instructions));
}

use std::fs::read_to_string;

fn parse_input(input: String) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
  let mut parts = input.split("\n\n");
  let (Some(crates_part), Some(instructions))
            = (parts.next(), parts.next()) else {
        panic!("Must have crates and instructions");
    };

  // crates
  let mut crates_lines = crates_part.lines().rev();
  let num_columns = crates_lines
    .next()
    .unwrap()
    .split_ascii_whitespace()
    .filter_map(|x| x.parse::<usize>().ok())
    .next_back()
    .unwrap();
  let mut crates = vec![vec![]; num_columns];
  for line in crates_lines {
    let mut chars = line.chars().skip(1);
    for i in 0..num_columns {
      if let Some(v) = chars.next() {
        if !v.is_ascii_whitespace() {
          crates[i].push(v);
        }
      }
      chars.next();
      chars.next();
      chars.next();
    }
  }
  // instructions
  let instructions = instructions
    .lines()
    .map(|line| {
      let mut numbers = line
        .split_ascii_whitespace()
        .filter_map(|word| word.parse().ok());
      match (numbers.next(), numbers.next(), numbers.next()) {
        (Some(x), Some(y), Some(z)) => (x, y, z),
        _ => panic!("instruction lines must have 3 numbers"),
      }
    })
    .collect();
  (crates, instructions)
}

fn part_1(crates: &Vec<Vec<char>>, instructions: &Vec<(usize, usize, usize)>) -> String {
  let mut crates = crates.clone();
  for &(n, from, to) in instructions {
    for _ in 0..n {
      if let Some(v) = crates[from - 1].pop() {
        crates[to - 1].push(v);
      }
    }
  }
  crates.iter().filter_map(|stack| stack.last()).collect()
}

fn part_2(crates: &Vec<Vec<char>>, instructions: &Vec<(usize, usize, usize)>) -> String {
  let mut crates = crates.clone();
  for &(n, from, to) in instructions {
    let new_size = crates[from - 1].len() - n;
    let mut tail = crates[from - 1][new_size..].iter().map(|c| *c).collect();
    crates[to - 1].append(&mut tail);
    crates[from - 1].truncate(new_size);
  }
  crates.iter().filter_map(|stack| stack.last()).collect()
}

fn main() {
  let (crates, instructions) = parse_input(read_to_string("input.txt").unwrap());

  println!("Part 1: {}", part_1(&crates, &instructions));
  println!("Part 2: {}", part_2(&crates, &instructions));
}

use std::fs::read_to_string;

type Posn = (i64, i64);
type Rock = Vec<Posn>;

fn rock(bl: Posn, idx: usize) -> Rock {
  match idx {
    0 => vec![bl, (bl.0 + 1, bl.1), (bl.0 + 2, bl.1), (bl.0 + 3, bl.1)],
    1 => vec![
      (bl.0 + 1, bl.1),
      (bl.0, bl.1 + 1),
      (bl.0 + 2, bl.1 + 1),
      (bl.0 + 1, bl.1 + 2),
    ],
    2 => vec![
      bl,
      (bl.0 + 1, bl.1),
      (bl.0 + 2, bl.1),
      (bl.0 + 2, bl.1 + 1),
      (bl.0 + 2, bl.1 + 2),
    ],
    3 => vec![bl, (bl.0, bl.1 + 1), (bl.0, bl.1 + 2), (bl.0, bl.1 + 3)],
    4 => vec![bl, (bl.0, bl.1 + 1), (bl.0 + 1, bl.1), (bl.0 + 1, bl.1 + 1)],
    _ => panic!("at the disco"),
  }
}

fn moved(r: &Rock, mv: char) -> Rock {
  r.iter()
    .map(|p| match mv {
      '<' => (p.0 - 1, p.1),
      '>' => (p.0 + 1, p.1),
      _ => (p.0, p.1 - 1),
    })
    .collect()
}

fn valid(rocks: &Rock, r: &Rock) -> bool {
  r.iter().all(|(x, y)| (0..7).contains(x) && *y > 0)
    && !rocks.iter().rev().any(|pos| r.contains(pos))
}

fn place_rock(jets: &[char], rocks: &mut Rock, mut r: Rock) -> i64 {
  let mut j = jets.iter();
  loop {
    if let Some(jet) = j.next() {
      let next = moved(&r, *jet);
      if valid(rocks, &next) {
        r = next;
      }
    }
    let down = moved(&r, '_');
    if valid(rocks, &down) {
      r = down;
    } else {
      let h = r.iter().map(|&(_, y)| y).max().unwrap();
      rocks.append(&mut r);
      return h;
    }
  }
}

fn print_rocks(rocks: &Rock) {
  println!(
    "{}",
    (0..=rocks.iter().map(|p| p.1).max().unwrap())
      .map(|y| {
        (0..7)
          .map(|x| if rocks.contains(&(x, y)) { '#' } else { ' ' })
          .collect::<String>()
          + "\n"
      })
      .rev()
      .collect::<String>()
  )
}

fn part_1(jets: Vec<char>) -> i64 {
  let mut rocks = vec![];
  let mut height = 0;
  for i in 0..2022 {
    height = place_rock(&jets, &mut rocks, rock((2, height + 3), i % 5));
    println!("-----------------------------");
    print_rocks(&rocks);
    println!("-----------------------------");

    // println!("-");
  }
  height
}

fn main() {
  let input: Vec<_> = read_to_string("example.txt").unwrap().chars().collect();
  println!("Part 1: {}", part_1(input));
}

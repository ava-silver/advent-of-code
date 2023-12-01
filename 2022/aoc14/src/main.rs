use std::{collections::HashSet, fs::read_to_string};

use itertools::Itertools;

type Posn = (usize, usize);

fn parse_layout(input: String) -> HashSet<Posn> {
  input
    .lines()
    .flat_map(|line| {
      line
        .split(" -> ")
        .map(|pair| {
          let mut tok = pair.split(',').filter_map(|x| x.parse::<usize>().ok());
          (tok.next().unwrap(), tok.next().unwrap())
        })
        .tuple_windows()
        .flat_map(|((x1, y1), (x2, y2))| {
          if x1 == x2 {
            (y1.min(y2)..=y1.max(y2))
              .map(|y| (x1, y))
              .collect::<Vec<_>>()
          } else {
            (x1.min(x2)..=x1.max(x2))
              .map(|x| (x, y1))
              .collect::<Vec<_>>()
          }
        })
    })
    .collect()
}

fn fall(layout: &HashSet<(usize, usize)>, sand_location: (usize, usize)) -> Option<Posn> {
  fn try_location(layout: &HashSet<(usize, usize)>, loc: Posn) -> Option<Posn> {
    if !layout.contains(&loc) {
      Some(loc)
    } else {
      None
    }
  }
  try_location(layout, (sand_location.0, sand_location.1 + 1))
    .or(try_location(
      layout,
      (sand_location.0 - 1, sand_location.1 + 1),
    ))
    .or(try_location(
      layout,
      (sand_location.0 + 1, sand_location.1 + 1),
    ))
}

fn place_sand(layout: &HashSet<(usize, usize)>, abyss: usize) -> Option<(usize, usize)> {
  let mut sand_location = (500, 0);
  while let Some(new_location) = fall(layout, sand_location) {
    if sand_location.1 > abyss {
      return None;
    }
    sand_location = new_location;
  }
  Some(sand_location)
}

fn part_1(mut layout: HashSet<Posn>) -> usize {
  let mut sand_placed = 0;
  let abyss = layout.iter().map(|s| s.1).max().unwrap();
  while let Some(sand) = place_sand(&layout, abyss) {
    layout.insert(sand);
    sand_placed += 1;
  }
  sand_placed
}

fn place_sand2(layout: &HashSet<(usize, usize)>, lowest: usize) -> (usize, usize) {
  let mut sand_location = (500, 0);
  while let Some(new_location) = fall(layout, sand_location) {
    sand_location = new_location;
    if sand_location.1 >= lowest {
      break;
    }
  }
  sand_location
}

fn part_2(mut layout: HashSet<Posn>) -> usize {
  let mut sand_placed = 0;
  let lowest = layout.iter().map(|s| s.1).max().unwrap() + 1;
  while !layout.contains(&(500, 0)) {
    layout.insert(place_sand2(&layout, lowest));
    sand_placed += 1;
  }
  sand_placed
}

fn main() {
  let input = read_to_string("input.txt").unwrap();
  let layout = parse_layout(input);
  println!("Part 1: {}", part_1(layout.clone()));
  println!("Part 2: {}", part_2(layout));
}

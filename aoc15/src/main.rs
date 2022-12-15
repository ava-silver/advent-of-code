use std::fs::read_to_string;

use itertools::Itertools;

type Posn = (i64, i64);

fn parse_int(tok: &str) -> i64 {
  tok[2..tok.len() - 1].parse().unwrap()
}

fn dist(p1: Posn, p2: Posn) -> u64 {
  p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn parse_sensors(input: String) -> Vec<(Posn, Posn, u64)> {
  input
    .lines()
    .map(|line| {
      let mut tok = line.split(' ').skip(2);
      let sensor_x = parse_int(tok.next().unwrap());
      let sensor_y = parse_int(tok.next().unwrap());
      let mut tok = tok.skip(4);
      let beacon_x = parse_int(tok.next().unwrap());
      let beacon_y = tok.next().unwrap()[2..].parse().unwrap();
      (
        (sensor_x, sensor_y),
        (beacon_x, beacon_y),
        dist((beacon_x, beacon_y), (sensor_x, sensor_y)),
      )
    })
    .collect()
}

fn invalid_beacon(field: &Vec<(Posn, Posn, u64)>, pos: Posn) -> bool {
  !field.iter().any(|(_, b, _)| *b == pos) && field.iter().any(|&(s, _, d)| dist(s, pos) <= d)
}

fn sensor_x(sensor: Posn, sensor_dist: u64, y: i64, min: bool) -> Option<i64> {
  let remaining_dist = sensor_dist as i64 - sensor.1.abs_diff(y) as i64;
  if remaining_dist >= 0 {
    Some(sensor.0 + (remaining_dist * if min { -1 } else { 1 }))
  } else {
    None
  }
}

fn part_1(field: Vec<(Posn, Posn, u64)>) -> usize {
  let y = 2_000_000;
  let min_x = field
    .iter()
    .filter_map(|&(s, _, d)| sensor_x(s, d, y, true))
    .min()
    .unwrap();
  let max_x = field
    .iter()
    .filter_map(|&(s, _, d)| sensor_x(s, d, y, false))
    .max()
    .unwrap();
  (min_x..=max_x)
    .filter(|&x| invalid_beacon(&field, (x, y)))
    .count()
}

fn coalesce(mut l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
  l.sort_by_key(|r| r.0);
  l.into_iter()
    .coalesce(|r1, r2| {
      if (r1.0 <= r2.1 && r2.0 <= r1.1) || r1.1.abs_diff(r2.0) == 1 || r1.0.abs_diff(r2.1) == 1 {
        Ok((r1.0.min(r2.0), r1.1.max(r2.1)))
      } else {
        Err((r1, r2))
      }
    })
    .collect()
}

const MAX: i64 = 4_000_000;

fn find_pos(field: &Vec<(Posn, Posn, u64)>, y: i64) -> Option<i64> {
  let ranges = coalesce(
    field
      .iter()
      .filter_map(|&(s, _, d)| {
        sensor_x(s, d, y, true)
          .and_then(|min_x| sensor_x(s, d, y, false).and_then(|max_x| Some((min_x, max_x))))
      })
      .collect(),
  );
  if ranges.len() == 2 {
    Some(ranges[0].1 + 1)
  } else {
    None
  }
}

fn part_2(field: Vec<(Posn, Posn, u64)>) -> i64 {
  for y in 0..=MAX {
    if let Some(x) = find_pos(&field, y) {
      return MAX * x + y;
    }
  }
  -1
}

fn main() {
  let input = read_to_string("input.txt").unwrap();
  let field = parse_sensors(input);
  println!("Part 1: {}", part_1(field.clone()));
  println!("Part 2: {}", part_2(field));
}

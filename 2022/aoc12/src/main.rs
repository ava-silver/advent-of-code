use std::{
  cmp::Ordering,
  collections::{BinaryHeap, HashMap},
  fs::read_to_string,
};

type Posn = (i32, i32);

fn parse_hill(input: String) -> (Vec<Vec<i32>>, Posn, Posn) {
  let mut start = (0, 0);
  let mut end = (0, 0);

  (
    input
      .lines()
      .enumerate()
      .map(|(y, line)| {
        line
          .chars()
          .enumerate()
          .map(|(x, c)| match c {
            'S' => {
              start = (x as i32, y as i32);
              'a' as i32
            }
            'E' => {
              end = (x as i32, y as i32);
              'z' as i32
            }
            _ => c as i32,
          })
          .collect()
      })
      .collect(),
    start,
    end,
  )
}

fn in_bounds(hill: &Vec<Vec<i32>>, p: Posn) -> bool {
  p.1 >= 0 && p.1 < hill.len() as i32 && p.0 >= 0 && p.0 < hill[0].len() as i32
}

fn valid_step(hill: &Vec<Vec<i32>>, cur: Posn, next: Posn) -> bool {
  hill[next.1 as usize][next.0 as usize] - hill[cur.1 as usize][cur.0 as usize] >= -1
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
  dist: usize,
  position: Posn,
}
impl Ord for State {
  fn cmp(&self, other: &Self) -> Ordering {
    other
      .dist
      .cmp(&self.dist)
      .then_with(|| self.position.cmp(&other.position))
  }
}
impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

fn pathfind(hill: Vec<Vec<i32>>, start: Posn, end: Box<dyn Fn(Posn) -> bool>) -> usize {
  let mut dists: HashMap<Posn, usize> = HashMap::new();
  let mut heap = BinaryHeap::new();
  // We're at `start`, with a zero cost
  dists.insert(start, 0);
  heap.push(State {
    dist: 0,
    position: start,
  });

  // Examine the frontier with lower cost nodes first (min-heap)
  while let Some(State { dist, position }) = heap.pop() {
    if end(position) {
      return dist;
    }
    if dist > *dists.get(&position).unwrap_or(&usize::MAX) {
      continue;
    }

    for neighbor in [
      (position.0 - 1, position.1),
      (position.0 + 1, position.1),
      (position.0, position.1 - 1),
      (position.0, position.1 + 1),
    ] {
      if in_bounds(&hill, neighbor) && valid_step(&hill, position, neighbor) {
        let next = State {
          dist: dist + 1,
          position: neighbor,
        };
        if next.dist < *dists.get(&neighbor).unwrap_or(&usize::MAX) {
          heap.push(next);
          dists.insert(neighbor, dist + 1);
        }
      }
    }
  }
  // end not reachable
  panic!("at the disco");
}

fn main() {
  let input = read_to_string("input.txt").unwrap();
  let (hill, start, end) = parse_hill(input);
  println!(
    "Part 1: {}",
    pathfind(hill.clone(), end, Box::new(move |p| p == start))
  );
  println!(
    "Part 2: {}",
    pathfind(
      hill.clone(),
      end,
      Box::new(move |p| hill[p.1 as usize][p.0 as usize] == 'a' as i32)
    )
  );
}

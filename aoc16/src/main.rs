use std::{collections::HashMap, fs::read_to_string};

fn parse_graph(input: String) -> HashMap<String, (usize, Vec<String>)> {
  input
    .lines()
    .map(|line| {
      let valve = line.split(' ').skip(1).next().unwrap().to_owned();
      let neighbors = line
        .split("valves ")
        .skip(1)
        .next()
        .unwrap()
        .split(", ")
        .map(String::from)
        .collect();
      let flow = line
        .split("rate=")
        .skip(1)
        .next()
        .unwrap()
        .split(";")
        .next()
        .unwrap()
        .parse()
        .unwrap();
      (valve, (flow, neighbors))
    })
    .collect()
}

fn part_1(graph: HashMap<String, (usize, Vec<String>)>) -> usize {
  todo!();
}

// fn part_2(graph: HashMap<String, (usize, Vec<String>)>) -> usize {
//     todo!();
// }

fn main() {
  let input = read_to_string("input.txt").unwrap();
  let graph = parse_graph(input);
  println!("Part 1: {}", part_1(graph.clone()));
  // println!("Part 2: {}", part_2(graph));
}

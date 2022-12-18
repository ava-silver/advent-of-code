use std::{collections::HashMap, fs::read_to_string, iter::once, vec};

use itertools::Itertools;

#[derive(Clone, Debug)]
struct Node {
  flow: usize,
  neighbors: Vec<String>,
}

#[derive(Clone, Debug)]
struct FlowState {
  flow: usize,
  flow_rate: usize,
  open_nodes: Vec<String>,
}

impl FlowState {
  fn next(&self) -> Self {
    Self {
      flow: self.flow + self.flow_rate,
      flow_rate: self.flow_rate,
      open_nodes: self.open_nodes.clone(),
    }
  }

  fn open(&self, rate: usize, opened: &String) -> Self {
    let mut new_open = self.open_nodes.clone();
    new_open.push(opened.to_owned());
    Self {
      flow: self.flow + self.flow_rate,
      flow_rate: self.flow_rate + rate,
      open_nodes: new_open,
    }
  }
}

fn parse_graph(input: String) -> HashMap<String, Node> {
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
      (valve, Node { flow, neighbors })
    })
    .collect()
}

const MINS: usize = 30;
/*
OPT(i, j, flow) = max(...(flow + OPT(i-1, k, flow) for k in j.predecessors),
                flow + OPT(i-1, j, j.flow) if j is not already open )
OPT(0, AA, 0) = 0
*/
fn part_1(graph: HashMap<String, Node>) -> usize {
  let mut m = vec![HashMap::from([(
    "AA".to_owned(),
    FlowState {
      flow: 0,
      flow_rate: 0,
      open_nodes: vec![],
    },
  )])];
  for i in 1..=MINS {
    // check other nodes
    let min_opts = m[i - 1]
      .iter()
      .flat_map(|(prev_node, state)| {
        graph[prev_node]
          .neighbors
          .iter()
          .map(|neighbor| Some((neighbor.to_owned(), state.next())))
          .chain({
            if state.open_nodes.contains(prev_node) {
              once(None)
            } else {
              once(Some((
                prev_node.to_owned(),
                state.open(graph[prev_node].flow, prev_node),
              )))
            }
          })
      })
      .filter_map(|x| x)
      // .sorted_by_key(|(key, _)| key.to_owned())
      // .group_by(|(key, _)| key.to_owned())
      // .into_iter()
      // .map(|(node, group)| (node, group.into_iter().map(|(_, state)| )))
      // .coalesce(|(p1, p2)| {
      //   if p1.0 == p2.0 {
      //     Ok((
      //       p1.0,
      //       match (p1.1, p2.1) {
      //         // determine state by which will make more flow by the end? or is this even viable? because also open nodes matters
      //         // so we definitely have to use the previous M value to determine this
      //         _ => panic!("hi"),
      //       },
      //     ))
      //   } else {
      //     Err((p1, p2))
      //   }
      // })
      .collect();
    m.push(min_opts);
  }
  m[MINS].values().map(|v| v.flow).max().unwrap_or(0)
}

// fn part_2(graph: HashMap<String, (usize, Vec<String>)>) -> usize {
//     todo!();
// }

fn main() {
  let input = read_to_string("example.txt").unwrap();
  let graph = parse_graph(input);
  println!("Part 1: {}", part_1(graph.clone()));
  // println!("Part 2: {}", part_2(graph));
}

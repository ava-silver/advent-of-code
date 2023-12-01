use std::fs::read_to_string;

use serde_json::{from_str, json, Value};

fn parse_messages(input: String) -> Vec<(Value, Value)> {
  input
    .split("\n\n")
    .map(|lines| {
      let mut l = lines.lines().map(|line| from_str(line).unwrap());
      (l.next().unwrap(), l.next().unwrap())
    })
    .collect()
}

fn compare(left: &Vec<Value>, right: &Vec<Value>) -> Option<bool> {
  for (left_val, right_val) in left.iter().zip(right) {
    if let (Some(left_int), Some(right_int)) = (left_val.as_i64(), right_val.as_i64()) {
      if left_int < right_int {
        return Some(true);
      } else if left_int > right_int {
        return Some(false);
      }
    } else if let (Some(left_arr), Some(right_arr)) = (left_val.as_array(), right_val.as_array()) {
      if let Some(ans) = compare(left_arr, right_arr) {
        return Some(ans);
      }
    } else if let (Some(_), Some(right_arr)) = (left_val.as_i64(), right_val.as_array()) {
      let left_arr = vec![left_val.to_owned()];
      if let Some(ans) = compare(&left_arr, right_arr) {
        return Some(ans);
      }
    } else if let (Some(left_arr), Some(_)) = (left_val.as_array(), right_val.as_i64()) {
      let right_arr = vec![right_val.to_owned()];
      if let Some(ans) = compare(left_arr, &right_arr) {
        return Some(ans);
      }
    }
  }
  match (left.len(), right.len()) {
    (llen, rlen) if llen != rlen => Some(llen < rlen),
    _ => None,
  }
}

fn part_1(messages: Vec<(Value, Value)>) -> usize {
  messages
    .into_iter()
    .enumerate()
    .filter_map(|(idx, (m1, m2))| {
      let (Some(left), Some(right)) = (m1.as_array(), m2.as_array()) else { return None; };
      if let Some(true) = compare(left, right) {
        Some(idx + 1)
      } else {
        None
      }
    })
    .sum()
}

fn parse_messages_2(input: String) -> Vec<Vec<Value>> {
  input
    .lines()
    .filter(|&l| l != "")
    .filter_map(|m| {
      from_str(m)
        .ok()
        .and_then(|ar: Value| ar.as_array().and_then(|arr| Some(arr.to_owned())))
    })
    .chain([vec![json!(vec![2])], vec![json!(vec![6])]])
    .collect()
}

fn marker_pos(messages: &Vec<Vec<Value>>, n: i64) -> usize {
  messages
    .iter()
    .position(|m| {
      m.len() == 1
        && matches!(
          m[0]
            .as_array()
            .and_then(|inner| { Some(matches!(inner.as_slice(), [v] if v == n)) }),
          Some(true)
        )
    })
    .unwrap()
    + 1
}

fn part_2(mut messages: Vec<Vec<Value>>) -> usize {
  messages.sort_by(|left, right| match compare(left, right) {
    Some(true) => std::cmp::Ordering::Less,
    Some(false) => std::cmp::Ordering::Greater,
    None => std::cmp::Ordering::Equal,
  });
  marker_pos(&messages, 2) * marker_pos(&messages, 6)
}

fn main() {
  let input = read_to_string("input.txt").unwrap();
  let messages = parse_messages(input.clone());
  println!("Part 1: {}", part_1(messages.clone()));
  let messages = parse_messages_2(input);
  println!("Part 2: {}", part_2(messages));
}

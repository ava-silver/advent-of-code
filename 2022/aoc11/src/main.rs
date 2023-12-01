use std::{fs::read_to_string, iter::Skip, str::Lines};

struct Monkey {
  items: Vec<usize>,
  operation: Box<dyn Fn(usize) -> usize>,
  test: Box<dyn Fn(usize) -> usize>,
  items_inspected: usize,
}

fn get_num(lines: &mut Skip<Lines>, split: &str) -> usize {
  lines
    .next()
    .unwrap()
    .split(split)
    .skip(1)
    .next()
    .unwrap()
    .parse()
    .unwrap()
}

// also produces the product of all the divisors, for use later
fn parse_monkeys(input: String) -> (Vec<Monkey>, usize) {
  let mut product = 1;
  (
    input
      .split("\n\n")
      .map(|monkey_text| {
        let mut lines = monkey_text.lines().skip(1);
        let items = lines
          .next()
          .unwrap()
          .split(": ")
          .skip(1)
          .next()
          .unwrap()
          .split(", ")
          .filter_map(|x| x.parse().ok())
          .collect();
        let mut op_tokens = lines
          .next()
          .unwrap()
          .split("new = ")
          .skip(1)
          .next()
          .unwrap()
          .split(' ')
          .collect::<Vec<_>>();
        let op = match op_tokens.remove(1) {
          "+" => usize::wrapping_add,
          "*" => usize::wrapping_mul,
          _ => panic!("at the disco"),
        };
        let v0 = op_tokens[0].parse().ok();
        let v1 = op_tokens[1].parse().ok();

        let modulus = get_num(&mut lines, "divisible by ");
        product *= modulus;
        let true_v = get_num(&mut lines, "to monkey ");
        let false_v = get_num(&mut lines, "to monkey ");

        Monkey {
          items,
          operation: Box::new(move |old| op(v0.unwrap_or(old), v1.unwrap_or(old))),
          test: Box::new(move |old| if old % modulus == 0 { true_v } else { false_v }),
          items_inspected: 0,
        }
      })
      .collect(),
    product,
  )
}

fn part_1(monkeys: &mut Vec<Monkey>) -> usize {
  for _ in 0..20 {
    // do round
    for i in 0..monkeys.len() {
      monkeys[i].items.reverse();
      while let Some(item) = monkeys[i].items.pop() {
        let worry = (monkeys[i].operation)(item) / 3;
        let next_monkey = (monkeys[i].test)(worry);
        monkeys[i].items_inspected += 1;
        monkeys[next_monkey].items.push(worry);
      }
    }
  }
  monkeys.sort_by_key(|m| m.items_inspected);
  monkeys.pop().unwrap().items_inspected * monkeys.pop().unwrap().items_inspected
}

fn part_2(monkeys: &mut Vec<Monkey>, product: usize) -> usize {
  for _ in 1..=10000 {
    // do round
    for i in 0..monkeys.len() {
      monkeys[i].items.reverse();
      while let Some(item) = monkeys[i].items.pop() {
        let worry = (monkeys[i].operation)(item);
        let next_monkey = (monkeys[i].test)(worry);
        monkeys[i].items_inspected += 1;
        monkeys[next_monkey].items.push(worry % product);
      }
    }
  }
  monkeys.sort_by_key(|m| m.items_inspected);
  monkeys.pop().unwrap().items_inspected * monkeys.pop().unwrap().items_inspected
}

fn main() {
  let input = read_to_string("input.txt").unwrap();
  let (mut monkeys, _) = parse_monkeys(input.clone());
  println!("Part 1: {}", part_1(&mut monkeys));
  let (mut monkeys, product) = parse_monkeys(input.clone());
  println!("Part 2: {}", part_2(&mut monkeys, product));
}

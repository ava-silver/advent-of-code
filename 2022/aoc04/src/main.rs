use std::fs::read_to_string;

fn elf_ranges(input: String) -> Vec<Vec<Vec<usize>>> {
  input
    .lines()
    .map(|line| {
      line
        .split(',')
        .map(|range| range.split('-').map(|d| d.parse().unwrap()).collect())
        .collect()
    })
    .collect()
}

fn subrange(range1: &Vec<usize>, range2: &Vec<usize>) -> bool {
  if let (Some(&s1), Some(&e1), Some(&s2), Some(&e2)) =
    (range1.get(0), range1.get(1), range2.get(0), range2.get(1))
  {
    return s1 >= s2 && e1 <= e2;
  }
  false
}

fn elf_subranges(elf_ranges: &Vec<Vec<Vec<usize>>>) -> usize {
  elf_ranges
    .iter()
    .filter(|&ranges| match (ranges.get(0), ranges.get(1)) {
      (Some(r1), Some(r2)) => subrange(r1, r2) || subrange(r2, r1),
      _ => false,
    })
    .count()
}

fn overlap(range1: &Vec<usize>, range2: &Vec<usize>) -> bool {
  if let (Some(&s1), Some(&e1), Some(&s2), Some(&e2)) =
    (range1.get(0), range1.get(1), range2.get(0), range2.get(1))
  {
    return s1 <= e2 && e1 >= s2;
  }
  false
}

fn elf_overlap(elf_ranges: &Vec<Vec<Vec<usize>>>) -> usize {
  elf_ranges
    .iter()
    .filter(|&ranges| match (ranges.get(0), ranges.get(1)) {
      (Some(r1), Some(r2)) => overlap(r1, r2),
      _ => false,
    })
    .count()
}

fn main() {
  let input = read_to_string("input.txt").unwrap();
  let elf_ranges = elf_ranges(input);
  println!("Part 1: {}", elf_subranges(&elf_ranges));
  println!("Part 2: {}", elf_overlap(&elf_ranges));
}

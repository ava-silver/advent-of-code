import gleam/dict
import gleam/int
import gleam/list
import gleam/string
import simplifile.{read}

fn unwrap(r: Result(a, b)) -> a {
  let assert Ok(x) = r
  x
}

fn enumerate(l: List(a)) -> List(#(a, Int)) {
  list.index_map(l, fn(x, i) { #(x, i) })
}

fn parse(input: String) -> List(List(Int)) {
  string.split(input, "\n")
  |> list.map(fn(line) {
    string.to_graphemes(line)
    |> list.map(fn(d) { unwrap(int.parse(d)) })
  })
}

fn find_max_n(battery: List(Int), n: Int) -> String {
  case n {
    0 -> ""
    _ -> {
      let all_but_last = list.take(battery, list.length(battery) - { n - 1 })
      let assert Ok(#(first, first_idx)) =
        list.max(enumerate(all_but_last), fn(p1, p2) { int.compare(p1.0, p2.0) })

      int.to_string(first)
      <> list.drop(battery, first_idx + 1) |> find_max_n(n - 1)
    }
  }
}

pub fn main() {
  let test_input =
    "987654321111111
811111111111119
234234234234278
818181911112111"
  let real_input = read("input.txt") |> unwrap
  let batteries = parse(real_input)

  let part1 =
    batteries
    |> list.map(fn(battery) { find_max_n(battery, 2) |> int.parse |> unwrap })
    |> int.sum
  echo part1
  let part2 =
    batteries
    |> list.map(fn(battery) { find_max_n(battery, 12) |> int.parse |> unwrap })
    |> int.sum
  echo part2
}

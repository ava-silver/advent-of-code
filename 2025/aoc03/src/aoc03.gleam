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

pub fn main() {
  let _test_input =
    "987654321111111
811111111111119
234234234234278
818181911112111"
  let real_input = read("input.txt") |> unwrap
  let batteries = parse(real_input)

  let answer =
    list.map(batteries, fn(battery) {
      let all_but_last = list.take(battery, list.length(battery) - 1)
      let assert Ok(#(first, first_idx)) =
        list.max(enumerate(all_but_last), fn(p1, p2) { int.compare(p1.0, p2.0) })
      let second =
        battery
        |> list.drop(first_idx + 1)
        |> list.max(int.compare)
        |> unwrap
      { first * 10 } + second
    })
    |> int.sum
  echo answer
}

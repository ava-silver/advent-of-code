import gleam/int
import gleam/list
import gleam/string
import simplifile

fn unwrap(r: Result(a, b)) -> a {
  let assert Ok(x) = r
  x
}

fn parse(input: String) -> #(List(#(Int, Int)), List(Int)) {
  let assert [ranges, ingredients] = input |> string.split("\n\n")
  let parse_ints = list.map(_, fn(i) { unwrap(int.parse(i)) })
  #(
    ranges
      |> string.split("\n")
      |> list.map(fn(r) {
        let assert [s, e] = r |> string.split("-") |> parse_ints
        #(s, e)
      }),
    ingredients |> string.split("\n") |> parse_ints,
  )
}

fn is_fresh(i: Int, ranges: List(#(Int, Int))) -> Bool {
  list.any(ranges, fn(r) { r.0 <= i && r.1 >= i })
}

pub fn main() {
  let _test_input =
    "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
  let input = simplifile.read("input.txt") |> unwrap
  let #(fresh_ranges, ingredients) = parse(input)
  // part 1
  echo list.count(ingredients, is_fresh(_, fresh_ranges))
  // part 2
  let condensed_ranges =
    list.sort(fresh_ranges, fn(x, y) { int.compare(x.0, y.0) })
    |> list.fold([], fn(acc, cur) {
      case acc {
        [] -> [cur]
        [#(start, end), ..rest] if cur.0 > end -> [cur, #(start, end), ..rest]
        [#(start, end), ..rest] if cur.1 > end -> [#(start, cur.1), ..rest]
        _ -> acc
      }
    })
  echo list.fold(condensed_ranges, 0, fn(acc, r) { acc + { r.1 - r.0 } + 1 })
}

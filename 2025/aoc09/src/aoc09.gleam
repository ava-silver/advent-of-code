import gleam/int
import gleam/list
import gleam/string
import simplifile

type Pos =
  #(Int, Int)

fn parse(input: String) -> List(Pos) {
  input
  |> string.split("\n")
  |> list.map(fn(l) {
    let assert [a, b] = l |> string.split(",") |> list.filter_map(int.parse)
    #(a, b)
  })
}

pub fn main() {
  let test_input =
    "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
  let assert Ok(input) = simplifile.read("input.txt")
  let points = parse(input)
  // part 1
  let assert Ok(largest_area) =
    points
    |> list.combination_pairs
    |> list.map(fn(p) {
      { int.absolute_value(p.0.0 - p.1.0) + 1 }
      * { int.absolute_value(p.0.1 - p.1.1) + 1 }
    })
    |> list.max(int.compare)
  echo largest_area
}

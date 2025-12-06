import gleam/list
import gleam/set
import gleam/string
import simplifile

fn unwrap(r: Result(a, b)) -> a {
  let assert Ok(x) = r
  x
}

fn enumerate(l: List(a)) -> List(#(a, Int)) {
  list.index_map(l, fn(x, i) { #(x, i) })
}

fn parse(input: String) -> set.Set(#(Int, Int)) {
  string.split(input, "\n")
  |> enumerate
  |> list.flat_map(fn(l) {
    let #(line, y) = l
    string.to_graphemes(line)
    |> enumerate
    |> list.flat_map(fn(s) {
      case s {
        #("@", x) -> [#(x, y)]
        _ -> []
      }
    })
  })
  |> set.from_list
}

fn adjacent(p: #(Int, Int)) -> List(#(Int, Int)) {
  let #(x, y) = p
  [
    #(x - 1, y - 1),
    #(x - 1, y + 1),
    #(x + 1, y + 1),
    #(x + 1, y - 1),
    #(x, y - 1),
    #(x - 1, y),
    #(x, y + 1),
    #(x + 1, y),
  ]
}

fn accessible(rolls: set.Set(#(Int, Int))) -> set.Set(#(Int, Int)) {
  set.filter(rolls, fn(pos) {
    pos |> adjacent |> list.count(set.contains(rolls, _)) < 4
  })
}

fn remove_all(rolls: set.Set(#(Int, Int))) -> set.Set(#(Int, Int)) {
  let a = accessible(rolls)
  case set.size(a) {
    0 -> rolls
    _ -> rolls |> set.difference(a) |> remove_all
  }
}

pub fn main() {
  let test_input =
    "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
  let input = simplifile.read("input.txt") |> unwrap
  let rolls = parse(input)
  // part 1
  let accessible_rolls = accessible(rolls) |> set.size
  echo accessible_rolls
  // part 2
  let rolls_left = set.size(rolls) - { remove_all(rolls) |> set.size }
  echo rolls_left
}

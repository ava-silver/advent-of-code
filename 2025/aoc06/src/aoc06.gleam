import gleam/int
import gleam/list
import gleam/regexp
import gleam/string
import simplifile

fn parse_ops(ops: String) -> List(fn(List(Int)) -> Int) {
  string.to_graphemes(ops)
  |> list.filter_map(fn(l) {
    case l {
      "*" -> Ok(int.product)
      "+" -> Ok(int.sum)
      _ -> Error(Nil)
    }
  })
}

fn parse_a(input: String) -> List(#(fn(List(Int)) -> Int, List(Int))) {
  let assert [ops, ..values] = input |> string.split("\n") |> list.reverse
  let assert Ok(split_spaces) = regexp.from_string(" +")
  let value_list =
    values
    |> list.map(fn(line) {
      regexp.split(split_spaces, line)
      |> list.filter_map(int.parse)
    })
    |> list.transpose
  list.zip(parse_ops(ops), value_list)
}

fn parse_b(input: String) -> List(#(fn(List(Int)) -> Int, List(Int))) {
  let assert [ops, ..values] = input |> string.split("\n") |> list.reverse
  let value_list =
    values
    // back in the correct order (top to bottom)
    |> list.reverse
    // list of lists of letters
    |> list.map(string.to_graphemes)
    // by column instead of row
    |> list.transpose
    // remove whitespaces and join the numbers
    |> list.map(fn(n) { list.filter(n, fn(l) { l != " " }) |> string.join("") })
    // these next 3 lines do a hack to split on the empty strings, since there's no list.split_on like we have for strings
    |> string.join(" ")
    |> string.split("  ")
    |> list.map(string.split(_, " "))
    // finally parse all the numbers in each sublist
    |> list.map(list.filter_map(_, int.parse))
  list.zip(parse_ops(ops), value_list)
}

fn compute(problems: List(#(fn(List(Int)) -> Int, List(Int)))) -> Int {
  problems |> list.map(fn(p) { p.0(p.1) }) |> int.sum
}

pub fn main() {
  let test_input =
    "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
  let assert Ok(input) = simplifile.read("input.txt")
  // part 1
  let problems = parse_a(input)
  echo compute(problems)
  // part 2
  let problems = parse_b(input)
  echo compute(problems)
}

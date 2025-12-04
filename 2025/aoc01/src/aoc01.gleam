import gleam/int
import gleam/list
import gleam/string
import simplifile.{read}

fn unwrap(r: Result(a, b)) -> a {
  let assert Ok(val) = r
  val
}

/// Parse the input to a list of moves
fn parse() -> List(Int) {
  let assert Ok(input) = read("input.txt")

  string.split(input, "\n")
  |> list.map(fn(e) {
    case string.first(e) |> unwrap() {
      "L" -> -1
      _ -> 1
    }
    * {
      string.drop_start(e, 1)
      |> int.parse()
      |> unwrap()
    }
  })
}

pub fn main() {
  let data = parse()
  // part 1
  let #(_, positions) =
    list.map_fold(data, 50, fn(pos, move) {
      let assert Ok(new) = int.modulo(pos + move, 100)
      #(new, new)
    })
  // echo positions
  echo list.count(positions, fn(x) { x == 0 })
  // part 2
  let #(_, passed_zero_times) =
    list.map_fold(data, 50, fn(pos, move) {
      #(
        int.modulo(pos + move, 100) |> unwrap(),
        list.range(pos, pos + move)
          |> list.drop(1)
          |> list.count(fn(x) { int.modulo(x, 100) == Ok(0) }),
      )
    })
  echo int.sum(passed_zero_times)
}

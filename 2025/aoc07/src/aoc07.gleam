import gleam/dict
import gleam/int
import gleam/list
import gleam/pair
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

type Pos =
  #(Int, Int)

pub type Game {
  Game(start_col: Int, height: Int, splitters: set.Set(Pos))
}

fn parse(input: String) -> Game {
  let board =
    input
    |> string.split("\n")
    |> enumerate
    |> list.map(fn(line) {
      #(line.0 |> string.to_graphemes |> enumerate, line.1)
    })
  let start =
    board |> list.first |> unwrap |> pair.first |> list.key_find("S") |> unwrap
  let splitters =
    board
    |> list.flat_map(fn(row) {
      row.0
      |> list.filter_map(fn(i) {
        case i {
          #("^", col) -> Ok(#(col, row.1))
          _ -> Error(Nil)
        }
      })
    })
    |> set.from_list
  Game(start, list.length(board), splitters)
}

fn num_splits(
  cur_lasers: set.Set(Int),
  height: Int,
  max_height: Int,
  splitters: set.Set(#(Int, Int)),
) -> Int {
  case height > max_height {
    True -> 0
    False -> {
      {
        set.intersection(
          splitters,
          cur_lasers |> set.map(fn(laser) { #(laser, height) }),
        )
        |> set.size
      }
      + {
        cur_lasers
        |> set.to_list
        |> list.flat_map(fn(laser) {
          case set.contains(splitters, #(laser, height)) {
            True -> [laser - 1, laser + 1]
            False -> [laser]
          }
        })
        |> set.from_list
        |> num_splits(height + 1, max_height, splitters)
      }
    }
  }
}

fn total_lasers(
  cur_lasers: dict.Dict(Int, Int),
  height: Int,
  max_height: Int,
  splitters: set.Set(#(Int, Int)),
) -> Int {
  case height > max_height {
    True -> cur_lasers |> dict.values |> int.sum
    False ->
      cur_lasers
      |> dict.to_list
      |> list.map(fn(kv) {
        let #(col, num_lasers) = kv
        case set.contains(splitters, #(col, height)) {
          True ->
            dict.from_list([#(col - 1, num_lasers), #(col + 1, num_lasers)])
          False -> dict.from_list([#(col, num_lasers)])
        }
      })
      |> list.fold(dict.new(), fn(a, b) {
        dict.combine(a, b, fn(v1, v2) { v1 + v2 })
      })
      |> total_lasers(height + 1, max_height, splitters)
  }
}

pub fn main() {
  let test_input =
    ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
  let input = simplifile.read("input.txt") |> unwrap
  let game = parse(input)
  // part 1
  echo num_splits(
    set.from_list([game.start_col]),
    0,
    game.height,
    game.splitters,
  )
  // part 2
  echo total_lasers(
    dict.from_list([#(game.start_col, 1)]),
    0,
    game.height,
    game.splitters,
  )
}

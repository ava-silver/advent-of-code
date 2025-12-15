import gleam/int
import gleam/list
import gleam/set
import gleam/string
import simplifile

fn unwrap(r: Result(a, b)) -> a {
  let assert Ok(val) = r
  val
}

pub type Machine {
  Machine(
    lights: set.Set(Int),
    buttons: List(set.Set(Int)),
    requirements: List(Int),
  )
}

fn drop_ends(input: String) -> String {
  input
  |> string.drop_start(1)
  |> string.drop_end(1)
}

fn parse_int_list(input: String) -> List(Int) {
  input |> drop_ends |> string.split(",") |> list.filter_map(int.parse)
}

fn parse(input: String) -> List(Machine) {
  input
  |> string.split("\n")
  |> list.map(fn(line) {
    let assert [lights_str, ..rest] = string.split(line, " ")
    let lights =
      lights_str
      |> drop_ends
      |> string.to_graphemes
      |> list.index_fold([], fn(acc, l, i) {
        case l {
          "#" -> [i, ..acc]
          _ -> acc
        }
      })
      |> set.from_list
    let assert [requirements, ..buttons] = list.reverse(rest)
    Machine(
      lights,
      buttons
        |> list.reverse
        |> list.map(parse_int_list)
        |> list.map(set.from_list),
      requirements |> parse_int_list,
    )
  })
}

fn all_combinations(l: List(a)) -> List(List(a)) {
  list.range(1, list.length(l)) |> list.flat_map(list.combinations(l, _))
}

fn min_presses(
  machine: Machine,
  states: set.Set(List(Int)),
  seen: set.Set(List(Int)),
) -> Int {
  let new_states =
    states
    |> set.to_list
    |> list.flat_map(fn(state) {
      machine.buttons
      |> list.filter_map(fn(button) {
        let new_state =
          state
          |> list.index_map(fn(joltage, i) {
            case set.contains(button, i) {
              True -> joltage + 1
              False -> joltage
            }
          })
        case
          !set.contains(seen, new_state)
          && list.zip(new_state, machine.requirements)
          |> list.all(fn(s) { s.0 <= s.1 })
        {
          True -> Ok(new_state)
          False -> Error(Nil)
        }
      })
    })
    |> set.from_list

  case set.contains(new_states, machine.requirements) {
    True -> 1
    False -> 1 + min_presses(machine, new_states, set.union(seen, new_states))
  }
}

pub fn main() {
  let test_input =
    "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
  let input = simplifile.read("input.txt") |> unwrap
  let machines = parse(input)
  // part 1
  let minimum_presses =
    machines
    |> list.map(fn(machine) {
      machine.buttons
      |> all_combinations
      |> list.filter_map(fn(buttons) {
        case
          list.fold(buttons, set.new(), set.symmetric_difference)
          == machine.lights
        {
          True -> Ok(list.length(buttons))
          False -> Error(Nil)
        }
      })
      |> list.reduce(int.min)
      |> unwrap
    })
    |> int.sum
  echo minimum_presses
  // part 2
  let minimum_presses =
    machines
    |> list.map(fn(machine) {
      min_presses(
        machine,
        set.from_list([list.map(machine.requirements, fn(_) { 0 })]),
        set.new(),
      )
    })
    |> int.sum
  echo minimum_presses
}

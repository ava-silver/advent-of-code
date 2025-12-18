import gleam/dict
import gleam/int
import gleam/list
import gleam/string
import simplifile

fn parse(input: String) -> dict.Dict(String, List(String)) {
  input
  |> string.split("\n")
  |> list.map(fn(line) {
    let assert [key, values] = string.split(line, ": ")
    #(key, string.split(values, " "))
  })
  |> dict.from_list
}

fn traverse(cur: String, servers: dict.Dict(String, List(String))) -> Int {
  case dict.get(servers, cur) {
    Ok(outputs) -> outputs |> list.map(traverse(_, servers)) |> int.sum
    Error(_) if cur == "out" -> 1
    Error(_) -> 0
  }
}

type State =
  #(String, #(Bool, Bool))

fn traverse_reqs(
  cur: String,
  seen: dict.Dict(State, Int),
  reqs: #(Bool, Bool),
  servers: dict.Dict(String, List(String)),
) -> #(Int, dict.Dict(State, Int)) {
  case dict.get(seen, #(cur, reqs)) {
    _ if cur == "out" && reqs == #(True, True) -> #(1, dict.new())
    _ if cur == "out" -> #(0, dict.new())
    Ok(n) -> #(n, dict.new())
    Error(_) -> {
      let reqs = case cur {
        "dac" -> #(True, reqs.1)
        "fft" -> #(reqs.0, True)
        _ -> reqs
      }
      let assert Ok(outputs) = dict.get(servers, cur)
      list.fold(outputs, #(0, seen), fn(acc, next) {
        let #(total, seen) = acc
        let #(more, new_seen) = traverse_reqs(next, seen, reqs, servers)
        #(
          total + more,
          dict.merge(seen, new_seen) |> dict.insert(#(next, reqs), more),
        )
      })
    }
  }
}

pub fn main() {
  let test_input =
    "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"
  let assert Ok(input) = simplifile.read("input.txt")
  let servers = parse(input)
  // part 1
  echo traverse("you", servers)
  // part 2
  let #(total, _) = traverse_reqs("svr", dict.new(), #(False, False), servers)
  echo total
}

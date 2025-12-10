import gleam/float
import gleam/int
import gleam/list
import gleam/set
import gleam/string
import simplifile

fn unwrap(r: Result(a, b)) -> a {
  let assert Ok(x) = r
  x
}

type Pos =
  #(Int, Int, Int)

fn parse(input: String) -> List(Pos) {
  input
  |> string.split("\n")
  |> list.map(fn(line) {
    let assert [x, y, z] =
      line |> string.split(",") |> list.filter_map(int.parse)
    #(x, y, z)
  })
}

fn sq(x: Int) -> Int {
  x * x
}

fn distance(a: Pos, b: Pos) -> Float {
  unwrap(int.square_root(sq(b.0 - a.0) + sq(b.1 - a.1) + sq(b.2 - a.2)))
}

fn get_connections(points: List(Pos)) -> List(#(Pos, Pos)) {
  points
  |> list.combination_pairs
  |> list.sort(fn(p0, p1) {
    float.compare(distance(p0.0, p0.1), distance(p1.0, p1.1))
  })
}

fn connect_circuits(
  circuits: List(set.Set(Pos)),
  connection: #(Pos, Pos),
) -> List(set.Set(Pos)) {
  let #(p0, p1) = connection
  case
    list.find(circuits, set.contains(_, p0)),
    list.any(circuits, set.contains(_, p1))
  {
    Error(_), False -> [set.from_list([p0, p1]), ..circuits]
    Error(_), True ->
      circuits
      |> list.map(fn(c) {
        case set.contains(c, p1) {
          True -> set.insert(c, p0)
          False -> c
        }
      })
    Ok(_), False ->
      circuits
      |> list.map(fn(c) {
        case set.contains(c, p0) {
          True -> set.insert(c, p1)
          False -> c
        }
      })
    Ok(c2), True ->
      circuits
      |> list.filter_map(fn(c) {
        case set.contains(c, p0), set.contains(c, p1) {
          False, False -> Ok(c)
          False, True -> Ok(set.union(c, c2))
          True, False -> Error(Nil)
          True, True -> Ok(c)
        }
      })
  }
}

fn make_n_circuits(
  connections: List(#(Pos, Pos)),
  num_connections: Int,
) -> List(set.Set(Pos)) {
  connections
  |> list.take(num_connections)
  |> list.fold([], connect_circuits)
}

fn get_connecting_points(
  connections: List(#(Pos, Pos)),
  circuits: List(set.Set(Pos)),
  points: set.Set(Pos),
) -> #(Pos, Pos) {
  let assert [connection, ..rest_connections] = connections
  let new_circuits = connect_circuits(circuits, connection)
  case new_circuits {
    [c] if c == points -> connection
    _ -> get_connecting_points(rest_connections, new_circuits, points)
  }
}

pub fn main() {
  let test_input =
    "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
  let input = simplifile.read("input.txt") |> unwrap
  let points = parse(input)
  let connections = get_connections(points)
  // part 1
  let circuits = make_n_circuits(connections, 1000)
  let assert [a, b, c, ..] =
    circuits |> list.map(set.size) |> list.sort(fn(a, b) { int.compare(b, a) })
  echo a * b * c
  // part 2
  let #(p0, p1) =
    get_connecting_points(connections, [], points |> set.from_list)
  echo p0.0 * p1.0
}

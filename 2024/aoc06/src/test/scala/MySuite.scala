// For more information on writing tests, see
// https://scalameta.org/munit/docs/getting-started.html
import aoc06._
val testInput = """....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."""

class MySuite extends munit.FunSuite {
  test("part1") {
    assertEquals(part1(testInput), 41)
  }
  test("part2") {
    assertEquals(part2(testInput), 6)
  }
}

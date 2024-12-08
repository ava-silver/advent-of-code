// For more information on writing tests, see
// https://scalameta.org/munit/docs/getting-started.html
import aoc08._

val example = """............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"""
class MySuite extends munit.FunSuite {
  test("part1") {
    assertEquals(part1(example), 14)
  }
  test("part2") {
    assertEquals(part2(example), 0)
  }
}
// For more information on writing tests, see
// https://scalameta.org/munit/docs/getting-started.html

import aoc10._
val input = """89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"""
class MySuite extends munit.FunSuite {
  test("part1") {
    assertEquals(part1(input), 36)
  }
  test("part2") {
    assertEquals(part2(input), 81)
  }
}

// For more information on writing tests, see
// https://scalameta.org/munit/docs/getting-started.html

import aoc13._
val input = """94 34
22 67
8400 5400

26 66
67 21
12748 12176

17 86
84 37
7870 6450

69 23
27 71
18641 10279"""

class MySuite extends munit.FunSuite {
  test("part1") {
    assertEquals(part1(input), 480)
  }
  test("part2") {
    assertEquals(part2(input), 0)
  }
}

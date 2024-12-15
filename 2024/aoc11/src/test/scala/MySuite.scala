// For more information on writing tests, see
// https://scalameta.org/munit/docs/getting-started.html

import aoc11._
val input = "125 17"
class MySuite extends munit.FunSuite {
  test("part1") {
    assertEquals(part1(input), BigInt(55312))
  }
}

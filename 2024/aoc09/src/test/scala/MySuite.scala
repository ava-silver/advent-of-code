// For more information on writing tests, see
// https://scalameta.org/munit/docs/getting-started.html

import aoc09._
val input = "2333133121414131402"
class MySuite extends munit.FunSuite {
  test("part1") {
    assertEquals(part1(input), BigInt(1928))
  }
  test("part2") {
    assertEquals(part2(input), BigInt(2858))
  }
}

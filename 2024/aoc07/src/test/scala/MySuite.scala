// For more information on writing tests, see
// https://scalameta.org/munit/docs/getting-started.html
import aoc07._
val testInput = """190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"""

class MySuite extends munit.FunSuite {
  test("part1") {
    assertEquals(part1(testInput), BigInt(3749))
  }
  test("part2") {
    assertEquals(part2(testInput), BigInt(11387))
  }
}

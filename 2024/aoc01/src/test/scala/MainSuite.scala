import munit.FunSuite
import aoc01._

def testInput = """3   4
4   3
2   5
1   3
3   9
3   3"""

class MySuite extends FunSuite {
  test("part1") {
    assertEquals(part1(testInput), 11)
  }
  test("part2") {
    assertEquals(part2(testInput), 31)
  }
}

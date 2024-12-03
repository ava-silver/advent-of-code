import aoc02._
var input = """7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"""
class MySuite extends munit.FunSuite {
  test("part 1") {
    assertEquals(part1(input), 2)
  }
  test("part 2") {
    assertEquals(part2(input), 4)
  }
}

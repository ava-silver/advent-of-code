import munit.FunSuite
import aoc01._

val (l1, l2) = parse("""3   4
4   3
2   5
1   3
3   9
3   3""")

class MySuite extends FunSuite {
  test("part1") {
    assertEquals(part1(l1, l2), 11)
  }
  test("part2") {
    assertEquals(part2(l1, l2), 31)
  }
}

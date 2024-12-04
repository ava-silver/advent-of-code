import munit.FunSuite
import aoc03._

class MySuite extends FunSuite {
  test("part1") {
    assertEquals(
      part1(
        """xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"""
      ),
      161
    )
  }
  test("part2") {
    assertEquals(
      part2(
        """xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"""
      ),
      48
    )
  }
}

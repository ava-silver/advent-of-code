// For more information on writing tests, see
// https://scalameta.org/munit/docs/getting-started.html

import aoc12._
val input1 = """AAAA
BBCD
BBCC
EEEC"""
val input2 = """RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"""
val input3 = """OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"""

val input4 = """EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"""
val input5 = """
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"""

class MySuite extends munit.FunSuite {
  test("part1") {
    assertEquals(part1(input1), 140)
    assertEquals(part1(input2), 1930)
  }
  test("part2") {
    assertEquals(part2(input1), 80)
    assertEquals(part2(input2), 1206)
    assertEquals(part2(input3), 436)
    assertEquals(part2(input4), 236)
    assertEquals(part2(input5), 368)
  }
}

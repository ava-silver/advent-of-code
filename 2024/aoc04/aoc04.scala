//> using jvm 21
//> using scala 3.5.2
//> using dep org.scalameta::munit:1.0.3

val directions = (-1 to 1)
  .flatMap { dx =>
    (-1 to 1).map { dy =>
      (dx, dy)
    }
  }
  .filterNot(_ == (0, 0))

val xmas = "XMAS".toCharArray

def countXmas(chars: Array[Array[Char]], row: Int, col: Int): Int =
  directions.count { case (dx, dy) =>
    xmas.zipWithIndex.forall { case (char, idx) =>
      val r = row + (idx * dy)
      val c = col + (idx * dx)
      r >= 0 && r < chars.length && c >= 0 && c < chars(0).length &&
      chars(r)(c) == char
    }
  }

def part1(input: String): Int =
  val chars = input.linesIterator.map(_.toCharArray).toArray
  (0 until chars.length).map { row =>
    (0 until chars(0).length).map { col =>
      countXmas(chars, row, col)
    }.sum
  }.sum

val ms = Set('M', 'S')

def isCrossedMas(chars: Array[Array[Char]], row: Int, col: Int): Boolean =
  val diag1 = Set(chars(row - 1)(col - 1), chars(row + 1)(col + 1))
  val diag2 = Set(chars(row - 1)(col + 1), chars(row + 1)(col - 1))
  chars(row)(col) == 'A' && diag1 == ms && diag2 == ms

def part2(input: String): Int =
  val chars = input.linesIterator.map(_.toCharArray).toArray
  (1 until chars.length - 1).map { row =>
    (1 until chars(0).length - 1).count { col => isCrossedMas(chars, row, col) }
  }.sum

val shortSample =
  """MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX""".strip

class TestSuite extends munit.FunSuite {

  test("part 1") {
    assertEquals(part1(shortSample), 18)
  }

  test("part 2") {
    assertEquals(part2(shortSample), 9)
  }

}

@main def main(): Unit =
  val input = io.Source.fromFile("input.txt").mkString
  println(part1(input))
  println(part2(input))

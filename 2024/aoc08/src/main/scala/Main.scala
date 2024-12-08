package aoc08
import scala.io.Source.fromResource
import scala.runtime.ScalaRunTime.stringOf
@main def main(): Unit =
  val input = fromResource("input.txt").mkString
  println(part1(input))
  println(part2(input))

def parse(input: String): Array[Array[Char]] =
  input.split("\n").map(_.toCharArray)

def part1(input: String): Int =
  val map = parse(input)
  val (width, height) = (map(0).length, map.length)
  val antennas = map.flatten.filter(_ != '.').toSet
  val antennaLocations = antennas.map { antenna =>
    map.zipWithIndex.flatMap { case (row, y) =>
      row.zipWithIndex.flatMap { case (n, x) =>
        if n == antenna then Array((x, y)) else Array[(Int, Int)]()
      }
    }
  }.toArray
  val valid = antennaLocations
    .flatMap(
      _.combinations(2)
        .flatMap { case Array((x1, y1), (x2, y2)) =>
          Array((x2 + x2 - x1, y2 + y2 - y1), (x1 + x1 - x2, y1 + y1 - y2))
        }
    )
    .filter { case (x, y) => x > 0 && y > 0 && x <= width && y <= height }
    .toSet
  println(stringOf(valid))
  valid.size

def part2(input: String): Int = 0

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
  antennaLocations
    .flatMap(
      _.combinations(2)
        .flatMap { case Array((x1, y1), (x2, y2)) =>
          Array((x2 + x2 - x1, y2 + y2 - y1), (x1 + x1 - x2, y1 + y1 - y2))
        }
    )
    .filter { case (x, y) => x >= 0 && y >= 0 && x < width && y < height }
    .toSet
    .size

def combinePoints(
    f: (Int, Int) => Int,
    p1: (Int, Int),
    p2: (Int, Int)
): (Int, Int) =
  (f(p1._1, p2._1), f(p1._2, p2._2))

def part2(input: String): Int =
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

  def allPointsInBounds(delta: (Int, Int), startingPoint: (Int, Int)) =
    LazyList
      .from(0)
      .map { i =>
        combinePoints(_ + _, startingPoint, combinePoints(_ * _, (i, i), delta))
      }
      .takeWhile { case (x, y) =>
        x >= 0 && y >= 0 && x < width && y < height
      }
      .toArray

  antennaLocations
    .flatMap(
      _.combinations(2)
        .flatMap { case Array(p1, p2) =>
          val d1 = combinePoints(_ - _, p1, p2)
          val d2 = combinePoints(_ - _, p2, p1)
          allPointsInBounds(d1, p1) ++ allPointsInBounds(d2, p2)
        }
    )
    .toSet
    .size

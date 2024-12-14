package aoc10

@main def main(): Unit =
  val input = fromResource("input.txt").mkString
  println(part1(input))
  println(part2(input))

def parse(input: String): Array[Array[Int]] =
  input.split("\n").map(_.toCharArray.map(_.toInt))

val directions = Array((0, 1), (1, 0), (-1, 0), (0, -1))

def addPoints(p1: (Int, Int), p2: (Int, Int)): (Int, Int) =
  (p1._1 + p2._1, p1._2 + p2._2)

def findApexes(map: Array[Array[T]], p0: (Int, Int)): Set[(Int, Int)] =
  val height = map(p0._2)(p0._1)
  if height == 9 then Set(p0)
  else
    directions
      .filter(p => (map(p._2)(p._1) - height) == 1)
      .flatMap(findApexes(map, _))
      .toSet

def part1(input: String): Int =
  val map = parse(input)
  val trailheads = map.zipWithIndex.flatMap { case (row, y) =>
    row.zipWithIndex.flatMap { case (height, x) =>
      if height == 0 then Array((x, y)) else Array()
    }
  }
  trailheads.flatMap(findApexes(map, _)).sum

def part2(input: String): Int = 0

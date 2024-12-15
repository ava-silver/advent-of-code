package aoc10
import scala.io.Source.fromResource
import scala.runtime.ScalaRunTime.stringOf
@main def main(): Unit =
  val input = fromResource("input.txt").mkString
  println(part1(input))
  println(part2(input))

def parse(input: String): Array[Array[Int]] =
  input.split("\n").map(_.split("").map(_.toInt))

val directions = Array((0, 1), (1, 0), (-1, 0), (0, -1))

def addPoints(p1: (Int, Int), p2: (Int, Int)): (Int, Int) =
  (p1._1 + p2._1, p1._2 + p2._2)

def findTrailheads(map: Array[Array[Int]]): Array[(Int, Int)] =
  map.zipWithIndex.flatMap { case (row, y) =>
    row.zipWithIndex.flatMap { case (height, x) =>
      if height == 0 then Array((x, y)) else Array[(Int, Int)]()
    }
  }
def findApexes(map: Array[Array[Int]], p0: (Int, Int)): Set[(Int, Int)] =
  val height = map(p0._2)(p0._1)
  val (mapW, mapH) = (map(0).length, map.length)
  if height == 9 then Set(p0)
  else
    directions
      .map(addPoints(p0, _))
      .filter { case (x, y) =>
        x >= 0 && y >= 0 && x < mapW && y < mapH
        && ((map(y)(x) - height) == 1)
      }
      .flatMap(findApexes(map, _))
      .toSet

def part1(input: String): Int =
  val map = parse(input)
  findTrailheads(map).map(findApexes(map, _).size).sum

def findPaths(map: Array[Array[Int]], p0: (Int, Int)): Int =
  val height = map(p0._2)(p0._1)
  val (mapW, mapH) = (map(0).length, map.length)
  if height == 9 then 1
  else
    directions
      .map(addPoints(p0, _))
      .filter { case (x, y) =>
        x >= 0 && y >= 0 && x < mapW && y < mapH
        && ((map(y)(x) - height) == 1)
      }
      .map(findPaths(map, _))
      .sum

def part2(input: String): Int =
  val map = parse(input)
  findTrailheads(map).map(findPaths(map, _)).sum

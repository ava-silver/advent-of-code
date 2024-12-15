package aoc12

import scala.io.Source.fromResource
import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer

@main def main(): Unit =
  val input = fromResource("input.txt").mkString
  println(part1(input))
  println(part2(input))

def parse(input: String): Array[Array[Char]] =
  input
    .split("\n")
    .map(_.toCharArray)

def groupByType(map: Array[Array[Char]]): Map[Char, Array[(Int, Int)]] =
  map.zipWithIndex
    .flatMap { case (row, y) =>
      row.zipWithIndex.map { case (c, x) =>
        c -> (x, y)
      }
    }
    .foldLeft(
      Map.empty[Char, Array[(Int, Int)]].withDefault(_ => Array[(Int, Int)]())
    ) { case (map, (c, p)) =>
      map.updated(c, map(c) :+ p)
    }
val directions = Array((0, 1), (1, 0), (-1, 0), (0, -1))

def addPoints(p1: (Int, Int), p2: (Int, Int)): (Int, Int) =
  (p1._1 + p2._1, p1._2 + p2._2)

def getMap(map: Array[Array[Char]], p: (Int, Int)): Option[Char] =
  map.lift(p._2).flatMap(_.lift(p._1))

def perimiter(map: Array[Array[Char]], points: Array[(Int, Int)]): Int =
  val c = getMap(map, points(0))
  points.map { p =>
    directions.count(d => (getMap(map, addPoints(p, d)) != c))
  }.sum

def getRegions(
    points: Array[(Int, Int)]
): Array[Array[(Int, Int)]] =
  val pointsRemaining = points.to(mutable.Set)
  val regions = mutable.ArrayBuffer[Array[(Int, Int)]]()
  while (!pointsRemaining.isEmpty) {
    var region = mutable.Set[(Int, Int)]()
    val p = pointsRemaining.head
    pointsRemaining.remove(p)
    region.add(p)
    var regionFinished = false
    while (!regionFinished) {
      val additionalPoints =
        region.flatMap(p =>
          directions.map(addPoints(_, p)).filter(pointsRemaining.remove)
        )
      region.addAll(additionalPoints)
      regionFinished = additionalPoints.isEmpty
    }
    regions.append(region.toArray)
  }
  regions.toArray

def part1(input: String): Int =
  val map = parse(input)
  val pointsByChar = groupByType(map)
  pointsByChar.values
    .flatMap(getRegions)
    .map(r => perimiter(map, r) * r.length)
    .sum

def part2(input: String): Int = 0

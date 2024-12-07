package aoc06

import scala.io.Source.fromResource
import scala.annotation.tailrec
import scala.collection.mutable.ArrayBuffer

val dirMap = Map(0 -> (0, -1), 1 -> (1, 0), 2 -> (0, 1), 3 -> (-1, 0))

def addPos(p1: (Int, Int), p2: (Int, Int)): (Int, Int) =
  (p1._1 + p2._1, p1._2 + p2._2)

@main def main(): Unit =
  val input = fromResource("input.txt").mkString
  println(part1(input))
  println(part2(input))

def parse(input: String): (Array[Array[Char]], Int, Int) =
  val map = input.split("\n").map(_.toCharArray)
  val startY = map.indexWhere(_.contains('^'))
  val startX = map(startY).indexWhere(_ == '^')
  (map, startX, startY)

def part1(input: String): Int =
  val (map, startX, startY) = parse(input)
  val (height, width) = (map.length, map(0).length)
  @tailrec
  def move(x: Int, y: Int, dir: Int, places: Set[(Int, Int)]): Int =
    val (newX, newY) = addPos((x, y), dirMap(dir))
    if newX < 0 || newX >= width || newY < 0 || newY >= height
    then places.size
    else {
      val nextPlace = map(newY)(newX)
      if nextPlace == '#' then move(x, y, (dir + 1) % 4, places)
      else move(newX, newY, dir, places + ((newX, newY)))
    }
  move(startX, startY, 0, Set((startX, startY)))

def part2(input: String): Int =
  val (baseMap, startX, startY) = parse(input)
  val (height, width) = (baseMap.length, baseMap(0).length)
  @tailrec
  def isLoop(
      map: ArrayBuffer[ArrayBuffer[Char]],
      x: Int,
      y: Int,
      dir: Int,
      seen: Set[(Int, Int, Int)]
  ): Boolean =
    val (newX, newY) = addPos((x, y), dirMap(dir))
    if newX < 0 || newX >= width || newY < 0 || newY >= height then false
    else if (seen.contains((x, y, dir))) then true
    else {
      val nextPlace = map(newY)(newX)
      if nextPlace == '#' then {
        val newDir = (dir + 1) % 4
        isLoop(map, x, y, newDir, seen + ((x, y, dir)))
      } else isLoop(map, newX, newY, dir, seen + ((x, y, dir)))
    }

  baseMap.zipWithIndex.map { case (row, y) =>
    row.zipWithIndex.count { case (c, x) =>
      if c != '.' then false
      else
        val mapCopy = baseMap.map(_.to(ArrayBuffer)).to(ArrayBuffer)
        mapCopy(y)(x) = '#'
        isLoop(
          mapCopy,
          startX,
          startY,
          0,
          Set()
        )
    }
  }.sum

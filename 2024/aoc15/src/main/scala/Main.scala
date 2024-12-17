package aoc15

import scala.io.Source.fromResource
import scala.collection.mutable

@main def main(): Unit =
  val input = fromResource("input.txt").mkString
  println(part1(input))
  println(part2(input))

type Point = (Int, Int)

implicit class PointOps(val p: Point) extends AnyVal {
  def +(other: Point): Point = (p._1 + other._1, p._2 + other._2)
  def *(other: Int): Point = (p._1 * other, p._2 * other)
}

// returns the robot position, set of boxes (mutable) and set of walls, as well as the instructions
def parse(
    input: String
): (Point, mutable.Set[Point], Set[Point], Array[Char]) =
  val parts = input.split("\n\n")
  val map = parts(0).split("\n").map(_.toCharArray)
  val robotY = map.indexWhere(_.contains('@'))
  val robotX = map(robotY).indexOf('@')
  val boxes = mutable.HashSet[Point]()
  val walls = map.zipWithIndex.flatMap { case (row, y) =>
    row.zipWithIndex.flatMap { case (c, x) =>
      if c == 'O' then boxes.add((x, y))
      if c == '#' then Array((x, y)) else Array[Point]()
    }
  }.toSet
  (
    (robotX, robotY),
    boxes,
    walls,
    parts(1).replace("\n", "").toCharArray
  )

val instrToDir = Map(
  '<' -> (-1, 0),
  '^' -> (0, -1),
  '>' -> (1, 0),
  'v' -> (0, 1)
)

def move(
    robot: Point,
    boxes: mutable.Set[Point],
    walls: Set[Point],
    move: Point
): Point =
  def canMove(p: Point, move: Point): Boolean =
    val newP = p + move
    !walls.contains(newP) && (!boxes.contains(newP) || canMove(newP, move))

  def moveBoxes(p: Point, move: Point): Unit =
    val newP = p + move
    if boxes.remove(newP) then {
      moveBoxes(newP, move)
      boxes.add(newP + move)
    }

  val nextPlace = robot + move
  if canMove(robot, move) then {
    moveBoxes(robot, move)
    nextPlace
  } else {
    robot
  }

def part1(input: String): Int =
  val (robotStart, boxes, walls, instructions) = parse(input)
  var robot = robotStart
  for (instruction <- instructions) {
    robot = move(robot, boxes, walls, instrToDir(instruction))
  }
  boxes.map { case (x, y) => x + (y * 100) }.sum

def part2(input: String): Int = 0

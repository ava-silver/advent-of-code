package aoc15

import scala.io.Source.fromResource

@main def main(): Unit =
  val input = fromResource("input.txt").mkString
  println(part1(input, (101, 103)))
  part2(input)

type Point = (Int, Int)

implicit class PointOps(val p: Point) extends AnyVal {
  def +(other: Point): Point = (p._1 + other._1, p._2 + other._2)
  def *(other: Int): Point = (p._1 * other, p._2 * other)
  def %(other: Point): Point =
    ((other._1 + p._1) % other._1, (other._2 + p._2) % other._2)
}

case class Robot(position: Point, velocity: Point)

def parse(input: String): Array[Robot] =
  input.split("\n").map { line =>
    val toks = line
      .split(" ")
      .map(_.split("=")(1).split(",").map(_.toInt))
      .map(p => (p(0), p(1)))
    Robot(toks(0), toks(1))
  }

def move(r: Robot, bounds: Point): Robot =
  Robot((r.position + r.velocity) % bounds, r.velocity)

def part1(input: String, bounds: Point): Int =
  var robots = parse(input)
  for (_ <- 1 to 100) {
    robots = robots.map(move(_, bounds))
  }
  val (centerX, centerY) = (bounds._1 / 2, bounds._2 / 2)
  robots.count { r =>
    val (x, y) = r.position
    x > centerX && y > centerY
  } * robots.count { r =>
    val (x, y) = r.position
    x < centerX && y < centerY
  } * robots.count { r =>
    val (x, y) = r.position
    x < centerX && y > centerY
  } * robots.count { r =>
    val (x, y) = r.position
    x > centerX && y < centerY
  }

def printRobots(rs: Array[Robot], bounds: Point) =
  println(
    (0 until bounds._2)
      .map { y =>
        (0 until bounds._1).map { x =>
          {
            val c = rs.count(_.position == (x, y))
            if c == 0 then "  " else "OO"
          }
        }.mkString
      }
      .mkString("\n")
  )
def part2(input: String): Unit =
  var robots = parse(input)
  val bounds = (101, 103)
  for (i <- LazyList.from(1)) {
    robots = robots.map(move(_, bounds))
    if ((i - 79) % 101 == 0) then {
      println(f"-------------- Second $i --------------")
      printRobots(robots, bounds)
      Thread.sleep(150)
    }
  }

package aoc13

import scala.io.Source.fromResource
import scala.collection.mutable.ArrayBuffer
import spire.math.Rational

@main def main(): Unit =
  val input = fromResource("input.txt").mkString
  println(part1(input))
  println(part2(input))

type Point = (Int, Int)

implicit class PointOps(val p: Point) extends AnyVal {
  def +(other: Point): Point = (p._1 + other._1, p._2 + other._2)
  def *(other: Int): Point = (p._1 * other, p._2 * other)
}

case class ClawGame(buttonA: Point, buttonB: Point, prize: Point)

def parse(input: String): Array[ClawGame] =
  input.split("\n\n").map { section =>
    val lines = section
      .split("\n")
      .map(_.split(" ").map(_.toInt))
      .map(ints => (ints(0), ints(1)))
    ClawGame(lines(0), lines(1), lines(2))
  }

def requiredTokens(game: ClawGame): Int =
  var tokens = Int.MaxValue
  for {
    aPresses <- 0 to 100
    bPresses <- 0 to 100
  } {
    if ((game.buttonA * aPresses) + (game.buttonB * bPresses) == game.prize)
    then {
      tokens = tokens.min(aPresses * 3 + bPresses)
    }
  }
  tokens

def part1(input: String): Int =
  parse(input).map(requiredTokens).filter(_ != Int.MaxValue).sum

val offset = BigInt("10000000000000")

def part2(input: String): BigInt =
  parse(input).map { case ClawGame(buttonA, buttonB, (prizeX, prizeY)) =>
    // shamelessly copied from Noble's AoC Solution: https://github.com/Noble-Mushtak/Advent-of-Code/blob/main/2024/day13/solution2.py
    val (x, y) = (offset + prizeX, offset + prizeY)
    val ((a, c), (b, d)) = (buttonA, buttonB)
    val i = Rational(d, a * d - b * c) * x + Rational(-b, a * d - b * c) * y
    val j = Rational(-c, a * d - b * c) * x + Rational(a, a * d - b * c) * y
    if (i > 0 && j > 0 && i.isWhole && j.isWhole) then ((3 * i) + j).toBigInt
    else BigInt(0)
  }.sum

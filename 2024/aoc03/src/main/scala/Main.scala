package aoc03
import scala.io.Source._

val regex = raw"mul\((\d+),(\d+)\)".r

def part1(input: String): Int =
  regex
    .findAllMatchIn(input)
    .map { m =>
      m.group(1).toInt * m.group(2).toInt
    }
    .sum

def part2(input: String): Int =
  def parse(i: String, on: Boolean): Int =
    val parts = i.split(if on then raw"don't\(\)" else raw"do\(\)", 2)
    val total = if on then part1(parts(0)) else 0
    if parts.length == 1 then total
    else total + parse(parts(1), !on)
  parse(input.split("\n").mkString, true)

@main def main(): Unit =
  val input = fromResource("input.txt").mkString
  println(part1(input))
  println(part2(input))

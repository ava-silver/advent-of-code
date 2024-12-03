package aoc02
import scala.io.Source.fromResource
import scala.util.boundary
import scala.collection.View
import java.util.Collection

def parse(input: String): Array[Array[Int]] =
  input.split("\n").map(_.split(" ").map(_.toInt))

def valid(levels: Array[Int]): Boolean =
  var prev = levels(0)
  var dir = (levels(0) - levels(1)).sign
  var valid = true
  for (level <- levels.drop(1)) {
    val diff = prev - level
    valid = valid && diff.sign == dir && (1 to 3).contains(diff.abs)
    prev = level
  }
  valid

def part1(input: String): Int =
  parse(input).filter(valid).length

def part2(input: String): Int =
  parse(input)
    .filter(report =>
      (0 to report.length)
        .exists(i => valid(report.take(i) ++ report.drop(i + 1)))
    )
    .length

@main def hello(): Unit =
  var input = fromResource("input.txt").mkString
  println(part1(input))
  println(part2(input))

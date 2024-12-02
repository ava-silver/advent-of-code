package aoc01
import scala.io.Source._

def part1(input: String): Int =
  val (list1, list2) =
    input
      .split("\n")
      .map(_.split("\\s+").map(_.toInt))
      .unzip(nums => (nums(0), nums(1)))
  list1.sorted.zip(list2.sorted).map { case (n1, n2) => (n1 - n2).abs }.sum

def part2(input: String): Int =
  val (list1, list2) =
    input
      .split("\n")
      .map(_.split("\\s+").map(_.toInt))
      .unzip(nums => (nums(0), nums(1)))
  val freq = list2.groupBy(identity).mapValues(_.size).toMap
  list1.map(n => freq.getOrElse(n, 0) * n).sum

@main def main(): Unit =
  val txt = fromResource("input.txt").mkString
  println(part1(txt))
  println(part2(txt))

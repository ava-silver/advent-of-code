package aoc01
import scala.io.Source._

def parse(input: String): (Array[Int], Array[Int]) = input
  .split("\n")
  .map(_.split("\\s+").map(_.toInt))
  .unzip(nums => (nums(0), nums(1)))

def part1(list1: Array[Int], list2: Array[Int]): Int =
  list1.sorted.zip(list2.sorted).map { case (n1, n2) => (n1 - n2).abs }.sum

def part2(list1: Array[Int], list2: Array[Int]): Int =
  val freq = list2.groupBy(identity).mapValues(_.size).toMap
  list1.map(n => freq.getOrElse(n, 0) * n).sum

@main def main(): Unit =
  val txt = fromResource("input.txt").mkString
  val (list1, list2) = parse(txt)
  println(part1(list1, list2))
  println(part2(list1, list2))

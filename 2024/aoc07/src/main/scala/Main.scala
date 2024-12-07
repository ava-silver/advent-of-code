package aoc07

import scala.io.Source.fromResource

@main def main(): Unit =
  val input = fromResource("input.txt").mkString
  println(part1(input))
  println(part2(input))

def parse(input: String): (Array[(BigInt, Array[BigInt])]) =
  input.split("\n").map { line =>
    val parts = line.split(": ")
    (BigInt(parts(0)), parts(1).split(" ").map(_.toInt))
  }

def test(
    ans: BigInt,
    nums: Array[BigInt],
    ops: List[(BigInt, BigInt) => BigInt]
): Boolean =
  nums.drop(1).zip(ops).foldLeft(nums(0)) { case (res, (n, f)) =>
    f(res, n)
  } == ans

def permutationsWithRepetition[E](elements: List[E], n: Int): List[List[E]] =
  if (n == 1) elements.map(List(_))
  else {
    for {
      elem <- elements
      perm <- permutationsWithRepetition(elements, n - 1)
    } yield elem :: perm
  }

def calibrationResult(
    input: String,
    ops: List[Function2[BigInt, BigInt, BigInt]]
): BigInt =
  parse(input)
    .filter { case (ans, nums) =>
      permutationsWithRepetition(ops, nums.length - 1).exists(
        test(ans, nums, _)
      )
    }
    .map(_._1)
    .sum

val ops1: List[Function2[BigInt, BigInt, BigInt]] = List((_ * _), (_ + _))

def part1(input: String): BigInt =
  calibrationResult(input, ops1)
val ops2: List[Function2[BigInt, BigInt, BigInt]] =
  List(
    (_ * _),
    (_ + _),
    { (x: BigInt, y: BigInt) => BigInt(x.toString + y.toString) }
  )

def part2(input: String): BigInt =
  calibrationResult(input, ops2)

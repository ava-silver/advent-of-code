package aoc11

import scala.io.Source.fromResource

@main def main(): Unit =
  val input = fromResource("input.txt").mkString
  println(part1(input))
  println(part2(input))

// the counts of each stone
def parse(input: String): Map[BigInt, BigInt] =
  input.split(" ").map(r => BigInt(r) -> BigInt(1)).toMap

// If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
// If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
// If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
def blink(rocks: Map[BigInt, BigInt]): Map[BigInt, BigInt] =
  rocks
    .flatMap[(BigInt, BigInt)] { case (rockValue, count) =>
      if rockValue == 0 then Array(BigInt(1) -> count)
      else if rockValue.toString.length % 2 == 0 then
        val (r1, r2) = rockValue.toString
          .splitAt(rockValue.toString.length / 2)
        Array(BigInt(r1) -> count, BigInt(r2) -> count)
      else Array((rockValue * 2024) -> count)
    }
    .foldLeft(Map.empty.withDefaultValue(BigInt(0))) {
      case (rocksMap, (rockValue, count)) =>
        rocksMap.updated(rockValue, count + rocksMap(rockValue))
    }

def simulate(input: String, iterations: Int): BigInt =
  val rocks = parse(input)
  (0 until iterations)
    .foldLeft(rocks) { case (r, _) =>
      blink(r)
    }
    .values
    .sum

def part1(input: String): BigInt = simulate(input, 25)

def part2(input: String): BigInt = simulate(input, 75)

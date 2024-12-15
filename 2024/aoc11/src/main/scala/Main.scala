package aoc11

import scala.io.Source.fromResource

@main def main(): Unit =
  val input = fromResource("input.txt").mkString
  println(part1(input))
  println(part2(input))

def parse(input: String): Array[BigInt] = input.split(" ").map(r => BigInt(r))

// If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
// If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
// If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
def blink(rocks: Array[BigInt]): Array[BigInt] =
  rocks.flatMap { rock =>
    if rock == 0 then Array(1)
    else if rock.toString.length % 2 == 0 then
      rock.toString
        .splitAt(rock.toString.length / 2)
        .toArray
        .map(r => BigInt(r))
    else Array(rock * 2024)
  }

def part1(input: String): BigInt =
  val rocks = parse(input)
  (0 to 25)
    .foldLeft(rocks) { case (r, _) =>
      blink(r)
    }
    .length

def part2(input: String): BigInt = 0

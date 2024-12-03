package aoc02
import scala.io.Source.fromResource

def parse(input: String): List[List[Int]] =
  input.split("\n").map(_.split(" ").map(_.toInt).toList).toList

def valid(levels: List[Int]): Boolean =
  def validAcc(levels: List[Int], prev: Int, dir: Int): Boolean =
    levels match
      case head :: rest => {
        val diff = prev - head
        (diff.sign == dir && (1 to 3).contains(diff.abs)
        && validAcc(rest, head, dir))
      }
      case Nil => true
  levels match
    case first :: second :: rest =>
      validAcc(second :: rest, first, (first - second).sign)
    case _ => true

def part1(input: String): Int =
  var valids = parse(input).filter(valid)
  valids.length

@main def hello(): Unit =
  var input = fromResource("input.txt").mkString
  println(part1(input))

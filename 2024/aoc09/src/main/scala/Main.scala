package aoc09

import scala.io.Source.fromResource
import scala.collection.mutable.ArrayBuffer
import scala.runtime.ScalaRunTime.stringOf
import scala.collection.mutable.ArrayDeque
import aoc09.DiskItem.Space
import aoc09.DiskItem.File

@main def main(): Unit =
  val input = fromResource("input.txt").mkString
  // println(part1(input))
  println(part2(input))

def parse1(input: String): ArrayBuffer[Option[Int]] =
  var idx = 0
  var space = false
  input
    .split("")
    .flatMap { c =>
      val blocks = Array.fill(c.toInt) { if space then None else Some(idx) }
      if !space then idx += 1
      space = !space
      blocks
    }
    .to(ArrayBuffer)

def checksum(blocks: ArrayBuffer[Option[Int]]): BigInt =
  blocks.zipWithIndex.foldLeft(BigInt(0)) {
    case (res, (Some(x), idx)) =>
      res + BigInt(x * idx)
    case (_, (None, _)) => println("something went wrong"); -1
  }

def part1(input: String): BigInt =
  val blocks = parse1(input)
  while (blocks.contains(None)) {
    val idx = blocks.indexOf(None)
    blocks(idx) = blocks.last
    blocks.dropRightInPlace(1)
    while (blocks.last == None) {
      blocks.dropRightInPlace(1)
    }
  }
  checksum(blocks)

enum DiskItem:
  case Space(size: Int, moved: Boolean)
  case File(size: Int, moved: Boolean, value: Int)

def parse2(input: String): ArrayDeque[DiskItem] =
  var idx = 0
  var space = false
  input
    .split("")
    .map { c =>
      val block =
        if space then Space(c.toInt, false) else File(c.toInt, false, idx)
      if !space then idx += 1
      space = !space
      block
    }
    .to(ArrayDeque)

def part2(input: String): BigInt =
  val blocks = parse2(input)
  blocks.length

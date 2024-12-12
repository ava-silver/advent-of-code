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
  case Space(size: Int)
  case File(size: Int, moved: Boolean, value: Int)

def isMoved(i: DiskItem): Boolean = i match
  case File(_, moved, _) => moved
  case Space(_)          => true

def parse2(input: String): ArrayDeque[DiskItem] =
  var idx = 0
  var space = false
  input
    .split("")
    .map { c =>
      val block =
        if space then Space(c.toInt) else File(c.toInt, false, idx)
      if !space then idx += 1
      space = !space
      block
    }
    .to(ArrayDeque)

def putSpace(blocks: ArrayDeque[DiskItem], size: Int, idx: Int): Unit =
  (blocks(idx - 1), blocks.lift(idx + 1)) match
    case (File(_, _, _), Some(Space(spaceRSize))) =>
      blocks.remove(idx)
      blocks(idx) = Space(size + spaceRSize)
    case (Space(spaceLSize), Some(Space(spaceRSize))) =>
      blocks(idx) = Space(size + spaceLSize + spaceRSize)
      blocks.remove(idx + 1)
      blocks.remove(idx - 1)
    case (Space(spaceLSize), _) =>
      blocks(idx) = Space(size + spaceLSize)
      blocks.remove(idx - 1)
    case (File(_, _, _), _) =>
      blocks(idx) = Space(size)

def part2(input: String): BigInt =
  val blocks = parse2(input)
  while (!blocks.forall(isMoved)) {
    val fileIdx = blocks.lastIndexWhere(!isMoved(_))
    val File(fileSize, _, value) = blocks(fileIdx): @unchecked
    val spaceIdx = blocks.indexWhere {
      case File(_, _, _) => false
      case Space(size)   => fileSize <= size
    }
    if spaceIdx < 0 || spaceIdx > fileIdx then
      blocks(fileIdx) = File(fileSize, true, value)
    else {
      val Space(spaceSize) = blocks(spaceIdx): @unchecked
      val spaceRemaining = spaceSize - fileSize
      blocks(spaceIdx) = File(fileSize, true, value)
      putSpace(blocks, fileSize, fileIdx)
      if spaceRemaining > 0 then
        blocks.insert(spaceIdx + 1, Space(spaceRemaining))
    }
  }
  var idx = 0
  blocks.map {
    case Space(size) => idx += size; BigInt(0)
    case File(size, _, value) => {
      val v = BigInt(value)
      val sum = (idx until idx + size).map(_ * v).sum
      idx += size
      sum
    }
  }.sum

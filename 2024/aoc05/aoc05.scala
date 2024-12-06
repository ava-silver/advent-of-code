//> using jvm 21
//> using scala 3.5.2
//> using dep org.scalameta::munit:1.0.3

import scala.collection.mutable.Set
import scala.collection.mutable.ArrayBuffer

def parse(input: String): (Array[(Int, Int)], Array[Array[Int]]) =
  val parts = input.split("\n\n")
  (
    parts(0)
      .split("\n")
      .map { line =>
        val parts = line.split("\\|").map(_.toInt)
        (parts(0), parts(1))
      },
    parts(1).split("\n").map(_.split(",").map(_.toInt))
  )

def requiredPages(
    rules: Array[(Int, Int)],
    page: Int,
    pages: Array[Int]
): Array[Int] =
  rules.filter(_._2 == page).map(_._1).filter(pages.contains(_))

def part1(input: String): Int =
  val (rules, jobs) = parse(input)
  jobs
    .filter { pages =>
      val seen = Set[Int]()
      pages.forall { page =>
        val req = requiredPages(rules, page, pages)
        seen += page
        req.forall(seen.contains(_))
      }
    }
    .map(pages => pages(pages.length / 2))
    .sum

def part2(input: String): Int =
  val (rules, jobs) = parse(input)
  val badOrderedJobs = jobs
    .filterNot { pages =>
      val seen = Set[Int]()
      pages.forall { page =>
        val req = requiredPages(rules, page, pages)
        seen += page
        req.forall(seen.contains(_))
      }
    }
  badOrderedJobs
    .map { pages =>
      val newList = ArrayBuffer[Int]()
      var reqGraph = scala.collection.mutable
        .Map[Int, ArrayBuffer[Int]]()
        .addAll(pages.map { page =>
          val req = requiredPages(rules, page, pages)
          (page, req.to(ArrayBuffer))
        })
      while (newList.length < pages.length) {
        reqGraph.find(_._2.isEmpty) match
          case None => println("uh oh")
          case Some((page, _)) => {
            newList.append(page)
            reqGraph.remove(page)
            reqGraph.toArray.map { case (p, reqs) =>
              val idx = reqs.indexOf(page)
              if idx == -1 then {
                reqGraph += (p -> reqs)
              } else {
                reqs.remove(idx)
              }
            }
          }
      }
      newList
    }
    .map(pages => pages(pages.length / 2))
    .sum

val shortSample = """47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"""

class TestSuite extends munit.FunSuite {

  test("part 1") {
    assertEquals(part1(shortSample), 143)
  }

  test("part 2") {
    assertEquals(part2(shortSample), 123)
  }

}

@main def main(): Unit =
  val input = io.Source.fromFile("input.txt").mkString
  println(part1(input))
  println(part2(input))

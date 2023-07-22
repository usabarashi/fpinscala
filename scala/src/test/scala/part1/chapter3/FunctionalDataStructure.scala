import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.Assertions._

import part1.chapter3.*
import part1.chapter3.List.apply

class FunctionalDataStructureSuite extends munit.FunSuite {
  test("Exercise 3.1") {
    val result = List(1,2,3,4,5) match
        case List.Cons(x, List.Cons(2, List.Cons(4, _))) => x
        case List.Nil => 42
        case List.Cons(x, List.Cons(y, List.Cons(3, List.Cons(4, _)))) => x + y
        case List.Cons(h, t) => h + List.sum(t)
        // case _ => 101
    assertEquals(result, 3)
  }

  test("Exercise 3.2") {
    assertThrows[Exception] {
      List.tail(List())
    }
    assertEquals(List.tail(List(1,2,3)), List(2,3))
  }

  test("Exercise 3.3") {
    assertThrows[Exception] {
      List.setHead(List[Int](), 42)
    }
    assertEquals(List.setHead(List(1,2,3), 42), List(42, 2,3))
  }

  test("Exercise 3.4") {
    assertEquals(List.drop(List(), 42), List())
    assertEquals(List.drop(List(1,2,3,4,5), 3), List(4,5))
  }

  test("Exercise 3.5") {
    assertEquals(List.dropWhile(List(), (n: Int) => n < 3), List())
    assertEquals(List.dropWhile(List(1,2,3,4,5), (n: Int) => n < 3), List(3, 4, 5))
  }

  test("Exercise 3.6") {
    assertThrows[Exception] {
      List.init(List())
    }
    assertEquals(List.init(List(1,2,3,4,5)), List(1, 2, 3, 4))
  }

  test("Exercise 3.8") {
    assertEquals(List.foldRight(List(1, 2, 3), List.Nil, (l, r: List[Int] ) => List.Cons(l, r)), List(1, 2, 3))
  }

  test("Exercise 3.9") {
    assertEquals(List.length(List.Nil), 0)
    assertEquals(List.length(List(1, 2, 3)), 3)
  }

  test("Exercise 3.10") {
    assertEquals(List.foldLeft(List(1, 2, 3), 0, _ + _), 6)
  }

  test("Exercise 3.11") {
    assertEquals(List.sumLeft(List(1, 2, 3)), 6)
    assertEquals(List.productLeft(List(1.0, 2.0, 3.0)), 6.0)
    assertEquals(List.lengthLeft(List(1, 2, 3)), 3)
  }

  test("Exercise 3.12") {
    assertEquals(List.reverse(List(1, 2, 3)), List(3, 2, 1))
  }

  test("Exercise 3.13") {
    assertEquals(List.foldLeftFromRight(List(1, 2, 3), 0, _ + _), 6)
    assertEquals(List.foldRightFromLeft(List(1, 2, 3), 0, _ + _), 6)
  }

  test("Exercise 3.14") {
    assertEquals(List.appendRight(List(1, 2, 3), List(4, 5, 6)), List(1, 2, 3, 4, 5, 6))
  }

  test("Exercise 3.15") {
    assertEquals(List.concat(List(List(1, 2, 3), List(4, 5, 6))), List(1, 2, 3, 4, 5, 6))
  }

  test("Exercise 3.16") {
    assertEquals(List.incrementEach(List(1, 2, 3)), List(2, 3, 4))
  }

  test("Exercise 3.17") {
    assertEquals(List.doubleToString(List(1.0, 2.0, 3.0, 4.0, 5.0)), List("1.0", "2.0", "3.0", "4.0", "5.0"))
  }

  test("Exercise 3.18") {
    assertEquals(List.map(List(1, 2, 3, 4, 5), _ * 2), List(2, 4, 6, 8, 10))
  }

  test("Exercise 3.19") {
    assertEquals(List.filter(List(1, 2, 3, 4, 5), (x) => x % 2 != 0), List(1, 3, 5))
  }

  test("Exercise 3.20") {
    assertEquals(List.flatMap(List(1, 2, 3), (x) => List(x, x)), List(1, 1, 2, 2, 3, 3))
  }

  test("Exercise 3.21") {
    assertEquals(List.filterFromFlatMap(List(1, 2, 3), (x) => x % 2 != 0), List(1, 3))
  }

  test("Exercise 3.22") {
    assertEquals(List.addPairwise(List(1, 2, 3), List(4, 5, 6)), List(5, 7, 9))
  }

  test("Exercise 3.23") {
    assertEquals(List.zipWith(List(1, 2, 3), List(4, 5, 6), _ + _), List(5, 7, 9))
  }

  test("LISTS IN THE STANDARD LIBRARY") {
    assertEquals((List.take(List(1, 2, 3, 4, 5), 3)), List(1, 2, 3))
    assertEquals((List.takeWhile(List(1, 2, 3, 4, 5), _ <= 3)), List(1, 2, 3))
    assertEquals(List.forall(List(1, 2, 3, 4, 5), _ <= 3), false)
    assertEquals(List.forall(List(1, 2, 3, 4, 5), _ <= 5), true)
    assertEquals(List.exists(List(1, 2, 3, 4, 5), _ == 42), false)
    assertEquals(List.exists(List(1, 2, 3, 4, 5), _ == 3), true)
    assertEquals(List.scanLeft(List("a", "b", "c", "d", "e"), "", _ + _), List("", "a", "ab", "abc", "abcd", "abcde"))
    assertEquals(List.scanRight(List("a", "b", "c", "d", "e"), "", _ + _), List("abcde", "bcde", "cde", "de", "e", ""))
  }

  test("Exercise 3.24") {
    assertEquals(List.hasSubsequence(List(1, 2, 3, 4, 5), List(5, 4, 3)), false)
    assertEquals(List.hasSubsequence(List(1, 2, 3, 4, 5), List(2, 3, 4)), true)
  }

}

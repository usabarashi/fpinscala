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
        case _ => 101
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
}

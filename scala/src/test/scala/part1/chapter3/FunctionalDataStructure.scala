import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.Assertions._

import part1.chapter3.*

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
}
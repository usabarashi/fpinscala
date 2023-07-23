import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.Assertions._

import part1.chapter3.Tree.*

class TreeSuite extends munit.FunSuite {
  test("Exercise 3.25") {
    assertEquals(Branch(Branch(Leaf(1), Leaf(2)), Branch(Leaf(3), Leaf(4))).maximum, 4)
  }

   test("Exercise 3.26") {
    assertEquals(Branch((Leaf(1)), Branch(Leaf(2), Branch(Leaf(3), Leaf(3)))).depth, 3)
  }

  test("Exercise 3.27") {
    assertEquals(
      Branch((Leaf(1)), Branch(Leaf(2), Branch(Leaf(3), Leaf(3)))).map(_.toString()),
      Branch((Leaf("1")), Branch(Leaf("2"), Branch(Leaf("3"), Leaf("3"))))
    )
  }

}

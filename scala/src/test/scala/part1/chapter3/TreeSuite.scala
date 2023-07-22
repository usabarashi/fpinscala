import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.Assertions._

import part1.chapter3.Tree.*

class TreeSuite extends munit.FunSuite {
  test("Exercise 3.24") {
    assertEquals(Branch(Branch(Leaf(1), Leaf(2)), Branch(Leaf(3), Leaf(4))).maximum, 4)
  }
}

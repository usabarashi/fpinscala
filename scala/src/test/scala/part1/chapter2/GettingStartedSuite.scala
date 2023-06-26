import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.Assertions._

class GettingStartedSuite extends munit.FunSuite {
  test("Exercise 2.1") {
    assertEquals(fib(0), 0)
    assertEquals(fib(1), 1)
    assertEquals(fib(2), 1)
    assertEquals(fib(3), 2)
    assertEquals(fib(4), 3)
    assertEquals(fib(5), 5)
    assertEquals(fib(6), 8)
    assertEquals(fib(7), 13)
    assertEquals(fib(8), 21)
    assertEquals(fib(9), 34)
    assertThrows[IllegalArgumentException] {
      fib(-42)
    }
  }

  test("Exercise 2.2") {
    assertEquals(isSorted(Array(1, 2, 3), _ > _), true)
    assertEquals(isSorted(Array(1, 2, 1), _ > _), false)
    assertEquals(isSorted(Array(3, 2, 1), _ < _), true)
    assertEquals(isSorted(Array(1, 2, 3), _ < _), false)
  }
}

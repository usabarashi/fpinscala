import scala.{Option as _, Some as _, None as _}

import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.Assertions._

import part1.chapter4.Option
import part1.chapter4.Option.*

class OptionSuite extends munit.FunSuite {
  test("Exercise 4.1") {
    //assertEquals(None.map(_ + 1), None)
    assertEquals(Some(42).map(_ + 1), Some(43))
    //assertEquals(None.flatMap(_ + 1), None)
    assertEquals(Some(42).flatMap((value) => Some(value + 1)), Some(43))
    assertEquals(None.getOrElse(0), 0)
    assertEquals(Some(42).getOrElse(0), 42)
    assertEquals(None.orElse((() => Some(42))()), Some(42))
    assertEquals(Some(42).orElse((()=> Some(0))()), Some(42))
    //assertEquals(None.filter(_ <= 42), None)
    assertEquals(Some(42).filter(_ <= 42), Some(42))
  }

  test("Exercise 4.2") {
    assertEquals(Option.variance(Seq(1.0, 2.0, 3.0, 4.0, 5.0)), Some(2.0))
  }

  test("Exercise 4.3") {
    //assertEquals(Option.map2(None, Some("World!"))((a, b) => a + b), None)
    assertEquals(Option.map2(Some("Hello!"), None)((a, b) => a + b), None)
    assertEquals(Option.map2(Some("Hello!"), Some("World!"))((a, b) => a + b), Some("Hello!World!"))
  }

  test("Exercise 4.4") {
    assertEquals(Option.sequence(List(Some("Hello!"), None)), None)
    assertEquals(Option.sequence(List(Some("Hello!"), Some("World!"))), Some(List("Hello!", "World!")))
  }

  test("Exercise 4.5") {
    assertEquals(Option.sequenceFromTraverse(List(Some("Hello!"), None)), None)
    assertEquals(Option.sequenceFromTraverse(List(Some("Hello!"), Some("World!"))), Some(List("Hello!", "World!")))
  }
}

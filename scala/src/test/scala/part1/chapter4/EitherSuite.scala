import scala.{Either as _, Left as _, Right as _}

import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.Assertions._

import part1.chapter4.Either
import part1.chapter4.Either.*

class EitherSuite extends munit.FunSuite {
  test("Exercise 4.6") {
    val left = Left[String, Int]("error")
    val right = Right[String, Int](42)

    val leftMap = left.map(value => value)
    val rightMap = right.map(value => value)
    assertEquals(leftMap, left)
    assertEquals(rightMap, right)

    val leftFlatMap = left.flatMap(value => Left[String, Int]("error"))
    val rightFlatMap = right.flatMap(value => Right[String, Int](value))
    assertEquals(leftFlatMap, left)
    assertEquals(rightFlatMap, right)

    val leftOrElse = left.orElse(Left[String, Integer]("error"))
    val rightOrElse = right.orElse(Left[String, Integer]("error"))
    assertEquals(leftOrElse, left)
    assertEquals(rightOrElse, right)

    val leftMap2 = left.map2(left)(_ + _)
    val rightMap2 = right.map2(right)(_ + _)
    assertEquals(leftMap2, left)
    assertEquals(rightMap2, Right[String, Int](84))
  }

  test("Exercise 4.7") {
    assertEquals(Either.sequence(List(Right("Hello!"), Left(0))), Left(0))
    assertEquals(Either.sequence(List(Right("Hello!"), Right("World!"))), Right(List("Hello!", "World!")))
    assertEquals(Either.traverse(List(Right("Hello!"), Left(0)))(x => x), Left(0))
    assertEquals(Either.traverse(List(Right("Hello!"), Right("World!")))(x => x), Right(List("Hello!", "World!")))
  }
}

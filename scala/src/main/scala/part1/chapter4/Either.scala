package part1.chapter4

import scala.{Either as _, Left as _, Right as _}
import scala.util.control.NonFatal

enum Either[+E, +A]:
  case Left(value: E)
  case Right(value: A)

  def map[B](f: A => B): Either[E, B] =
    this match
      case Left(error) => Left(error)
      case Right(value) => Right(f(value))

  def flatMap[EE >: E, B](f: A => Either[EE, B]): Either[EE, B] =
    this match
      case Left(error) => Left(error)
      case Right(value) => f(value)

  def orElse[EE >: E,B >: A](b: => Either[EE, B]): Either[EE, B] =
    this match
      case Left(_) => b
      case Right(a) => Right(a)

  def map2[EE >: E, B, C](that: Either[EE, B])(f: (A, B) => C): Either[EE, C] =
    for
      a <- this
      b <- that
    yield f(a, b)

def mean(xs: Seq[Double]): Either[String, Double] =
  if xs.isEmpty then
    Either.Left("mean of empty list!")
  else
    Either.Right(xs.sum / xs.length)

def safeDiv(x: Int, y: Int): Either[Throwable, Int] =
  try Either.Right(x / y)
  catch case NonFatal(t) => Either.Left(t)

def catchNonFatal[A](a: => A): Either[Throwable, A] =
  try Either.Right(a)
  catch case NonFatal(t) => Either.Left(t)

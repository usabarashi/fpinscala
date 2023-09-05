package part1.chapter4

import scala.{Either, Left, Right}

case class Name private (value: String)
object Name:
    def apply(name: String): Either[String, Name] =
        if name == "" || name == null then Left("Name is empty.")
        else Right(new Name(name))

case class Age private (value: Int)
object Age:
    def apply(age: Int): Either[String, Age] =
        if age < 0 then Left("Age is out of range.")
        else Right(new Age(age))

case class Person(name: Name, age: Age)
object Person:
    def makeBoth(name: String, age: Int): Either[List[String], Person] =
        map2Both(Name(name), Age(age), Person(_, _))

def map2Both[E, A, B, C](a: Either[E, A], b: Either[E, B], f: (A, B) => C): Either[List[E], C] =
    (a, b) match
        case (Right(aa), Right(bb)) => Right(f(aa, bb))
        case (Left(e), Right(_)) => Left(List(e))
        case (Right(_), Left(e)) => Left(List(e))
        case (Left(e1), Left(e2)) => Left(List(e1, e2))

def map2All[E, A, B, C](a: Either[List[E], A], b: Either[List[E], B], f: (A, B) => C): Either[List[E], C] =
    (a, b) match
        case (Right(aa), Right(bb)) => Right(f(aa, bb))
        case (Left(es), Right(_)) => Left(es)
        case (Right(_), Left(es)) => Left(es)
        case (Left(es1), Left(es2)) => Left(es1 ++ es2)

def traverseAll[E, A, B](as: List[A], f: A => Either[List[E], B]): Either[List[E], List[B]] =
    as.foldRight(Right(Nil): Either[List[E], List[B]])((a, acc) => map2All(f(a), acc, _ :: _))

enum Validated[+E, +A]:
    case Invalid(errors: List[E])
    case Valid(get: A)

    def toEither: Either[List[E], A] =
        this match
            case Valid(a) => Right(a)
            case Invalid(es) => Left(es)

    def map[B](f: A => B): Validated[E, B] =
        this match
            case Valid(a) => Valid(f(a))
            case Invalid(es) => Invalid(es)

    def map2[EE >: E, B, C](b: Validated[EE, B])(f: (A, B) => C): Validated[EE, C] =
        (this, b) match
            case (Valid(aa), Valid(bb)) => Valid(f(aa, bb))
            case (Invalid(es), Valid(_)) => Invalid(es)
            case (Valid(_), Invalid(es)) => Invalid(es)
            case (Invalid(es1), Invalid(es2)) => Invalid(es1 ++ es2)

object Validated:
    def fromEither[E, A](e: Either[List[E], A]): Validated[E, A] =
        e match
            case Right(a) => Valid(a)
            case Left(es) => Invalid(es)

    def traverse[E, A, B](as: List[A], f: A => Validated[E, B]): Validated[E, List[B]] =
        as.foldRight(Valid(Nil): Validated[E, List[B]])((a, acc) => f(a).map2(acc)(_ :: _))

    def sequence[E, A](vs: List[Validated[E, A]]): Validated[E, List[A]] =
        traverse(vs, identity)

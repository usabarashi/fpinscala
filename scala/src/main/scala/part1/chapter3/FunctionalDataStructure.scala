package part1.chapter3

enum List[+A]:
  case Nil
  case Cons(head: A, tail: List[A])

object List:
  def sum(ints: List[Int]): Int = ints match
    case Nil => 0
    case Cons(x,xs) => x + sum(xs)

  def product(doubles: List[Double]): Double = doubles match
    case Nil => 1.0
    case Cons(0.0, _) => 0.0
    case Cons(x,xs) => x * product(xs)

  def apply[A](as: A*): List[A] =
    if as.isEmpty then Nil
    else Cons(as.head, apply(as.tail*))

  def tail[A](as: List[A]): List[A] = as match
    case Nil => sys.error("Does not have tail.")
    case Cons(_, tail) => tail

  def setHead[A](as: List[A], a: A): List[A] = as match
    case Nil => sys.error("Does not have head.")
    case Cons(_, tail) => Cons(head=a, tail=tail)

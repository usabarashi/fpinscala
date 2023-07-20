package part1.chapter3

enum List[+A]:
  case Nil
  case Cons(head: A, tail: List[A])

object List:
  def sum(ints: List[Int]): Int =
    ints match
      case Nil => 0
      case Cons(x,xs) => x + sum(xs)

  def product(doubles: List[Double]): Double =
    doubles match
      case Nil => 1.0
      case Cons(0.0, _) => 0.0
      case Cons(x,xs) => x * product(xs)

  def apply[A](as: A*): List[A] =
    if as.isEmpty then Nil
    else Cons(as.head, apply(as.tail*))

  def tail[A](as: List[A]): List[A] =
    as match
      case Nil => sys.error("Does not have tail.")
      case Cons(_, tail) => tail

  def setHead[A](as: List[A], a: A): List[A] =
    as match
      case Nil => sys.error("Does not have head.")
      case Cons(_, tail) => Cons(head=a, tail=tail)

  def drop[A](as: List[A], n: Int): List[A] =
    as match
      case Cons(_, tail) if 0 < n => drop(tail, n-1)
      case _  => as

  def dropWhile[A](as: List[A], f: A => Boolean): List[A] =
    as match
      case Cons(head, tail) if f(head) => dropWhile(tail, f)
      case _ => as

  def append[A](a1: List[A], a2: List[A]): List[A] =
    a1 match
      case Nil => a2
      case Cons(h, t) => Cons(h, append(t, a2))

  def init[A](as: List[A]): List[A] =
    as match
      case Nil => sys.error("Does not have tail.")
      case Cons(_, Nil) => Nil
      case Cons(head, tail) => Cons(head, init(tail))

  def foldRight[A, B](as: List[A], acc: B, f: (A, B) => B): B =
    as match
      case Nil => acc
      case Cons(x, xs) => f(x, foldRight(xs, acc, f))

  def length[A](as: List[A]): Int =
    foldRight(as, 0, (_, b) => b + 1)

  @annotation.tailrec
  def foldLeft[A, B](l: List[A], acc: B, f: (B, A) => B): B =
    l match
      case Nil => acc
      case Cons(head, tail) => foldLeft(tail, f(acc, head), f)

  def sumLeft(l: List[Int]): Int =
    foldLeft[Int, Int](l, 0, _ + _)

  def productLeft(l: List[Double]): Double =
    foldLeft[Double, Double](l, 1.0, _ * _)

  def lengthLeft[A](l: List[A]): Int =
    foldLeft[A, Int](l, 0, (b, _) => b + 1)

  def reverse[A](l: List[A]): List[A] =
    foldLeft(l, List[A](), (acc, h) => Cons(h, acc))

  def foldLeftFromRight[A, B](l: List[A], acc: B, f: (B, A) => B): B =
    foldRight(l, acc, (acc, h) => f(h, acc))

  def foldRightFromLeft[A, B](l: List[A], acc: B, f: (A, B) => B): B =
    foldLeft(l, acc, (h, acc) => f(acc, h))

  def appendRight[A](a1: List[A], a2: List[A]): List[A] =
    foldRight(a1, a2, Cons(_, _))

  def concat[A](l: List[List[A]]): List[A] =
    foldRight(l, Nil: List[A], append)

  def incrementEach(l: List[Int]): List[Int] =
    foldRight(l, Nil: List[Int], (i, acc) => Cons(i + 1, acc))

  def doubleToString(l: List[Double]): List[String] =
    foldRight(l, Nil: List[String], (d, acc) => Cons(d.toString, acc))

  def map[A, B](as: List[A], f: A => B): List[B] =
    foldRight(as, Nil: List[B], (a, acc) => Cons(f(a), acc))

  def filter[A](as: List[A], f: A => Boolean): List[A] =
    foldRight(as, Nil: List[A], (a, acc) => if (f(a)) Cons(a, acc) else acc)

  def flatMap[A, B](as: List[A], f: A => List[B]): List[B] =
    concat(map(as, f))

  def filterFromFlatMap[A](as: List[A], f: A => Boolean): List[A] =
    flatMap(as, (x) => if (f(x)) List(x) else Nil)

  def addPairwise(a: List[Int], b: List[Int]): List[Int] =
    (a, b) match
      case (Nil, _) | (_, Nil) => Nil
      case (Cons(h1, t1), Cons(h2, t2)) => Cons(h1+h2, addPairwise(t1, t2))

  def zipWith[A, B, C](a: List[A], b: List[B], f: (A, B) => C): List[C] =
    (a, b) match
      case (Nil, _) | (_, Nil) => Nil
      case (Cons(ah, at), Cons(bh, bt)) => Cons(f(ah, bh), zipWith(at, bt, f))

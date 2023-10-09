package part1.chapter5

enum LazyList[+A]:
    case Empty
    case Cons(h: () => A, t: () => LazyList[A])
        def headOption: Option[A] =
            this match
                case Empty => None
                case Cons(h, _) => Some(h())

    def toList: List[A] =
        @annotation.tailrec
        def go(ll: LazyList[A], acc: List[A]): List[A] =
            ll match
                case Cons(h, t) => go(t(), h() :: acc)
                case Empty => acc.reverse
        go(this, Nil)

object LazyList:
    def cons[A](
        hd: => A, tl: => LazyList[A]
    ): LazyList[A] =
        lazy val head = hd
        lazy val tail = tl
        Cons(() => head, () => tail)

    def empty[A]: LazyList[A] = Empty

    def apply[A](as: A*): LazyList[A] =
        if as.isEmpty
        then empty
        else
            lazy val head = as.head
            lazy val tail = as.tail
            Cons(() => head, () => apply(tail*))

package part1.chapter3

enum Tree[+A]:
    case Leaf(value: A)
    case Branch(left: Tree[A], right: Tree[A])

    def size: Int =
        this match
            case Leaf(_) => 1
            case Branch(l, r) => 1 + l.size + r.size

    def depth: Int =
        this match
            case Leaf(_) => 0
            case Branch(l, r) => 1 + (l.depth.max(r.depth))

    def map[B](f: A => B): Tree[B] =
        this match
            case Leaf(value) => Leaf(f(value))
            case Branch(left, right) => Branch(left.map(f), right.map(f))

    def fold[B](f: A => B, g: (B, B) => B): B =
        this match
            case Leaf(value) => f(value) // Equivalent to Nil and accumulator
            case Branch(left, right) => g(left.fold(f, g), right.fold(f, g))

    def sizeViaFold: Int =
        fold(a => 1, 1 + _ + _)

    def depthViaFold: Int =
        fold(a => 0, (left, right) => 1 + (left max right))

    def mapViaFold[B](f: A => B): Tree[B] =
        fold(value => Leaf(f(value)), Branch(_, _))

object Tree:
    def size[A](t: Tree[A]): Int =
        t match
            case Leaf(_) => 1
            case Branch(l, r) => 1 + size(l) + size(r)

    extension (t: Tree[Int]) def maximum: Int =
        t match
            case Leaf(value) => value
            case Branch(left, right) => left.maximum.max(right.maximum)

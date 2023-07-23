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

object Tree:
    def size[A](t: Tree[A]): Int =
        t match
            case Leaf(_) => 1
            case Branch(l, r) => 1 + size(l) + size(r)

    extension (t: Tree[Int]) def maximum: Int =
        t match
            case Leaf(n) => n
            case Branch(l, r) => l.maximum.max(r.maximum)

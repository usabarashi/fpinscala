package part1.chapter3

enum Tree[+A]:
    case Leaf(value: A)
    case Branch(leaf: Tree[A], right: Tree[A])

    def size: Int =
        this match
            case Leaf(_) => 1
            case Branch(l, r) => 1 + l.size + r.size
object Tree:
    def size[A](t: Tree[A]): Int =
        t match
            case Leaf(_) => 1
            case Branch(l, r) => 1 + size(l) + size(r)

    extension (t: Tree[Int]) def maximum: Int =
        t match
            case Leaf(n) => n
            case Branch(l, r) => l.maximum.max(r.maximum)

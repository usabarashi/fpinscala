from part1.chapter3.functional_data_structure import List, Nil, Cons


def test_exercise31():
    def result() -> int:
        match List(1, 2, 3, 4, 5):
            case Cons(x, Cons(2, Cons(4, _))):
                return x
            case Nil():
                return 42
            case Cons(x, Cons(y, Cons(3, Cons(4, _)))):
                return x + y
            case Cons(h, t):
                return h + t.sum()
            case _:
                return 101

    assert 3 == result()

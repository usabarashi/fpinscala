import pytest
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


def test_exercise32():
    with pytest.raises(EOFError):
        List[int]().tail
    assert List[int](2, 3) == List[int](1, 2, 3).tail


def test_exercise33():
    with pytest.raises(Exception):
        List[int]().set_head(42)
    assert List[int](42, 2, 3) == List[int](1, 2, 3).set_head(42)


def test_exercise34():
    assert List[int]() == List[int]().drop(42)
    assert List[int](4, 5) == List[int](1, 2, 3, 4, 5).drop(3)


def test_exercise35():
    assert List[int]() == List[int]().drop_while(lambda n: n < 42)
    assert List[int](4, 5) == List[int](1, 2, 3, 4, 5).drop_while(lambda n: n < 4)


def test_exercise36():
    with pytest.raises(EOFError):
        List[int]().init()
    assert List[int](1, 2, 3, 4) == List[int](1, 2, 3, 4, 5).init()


def test_exercise38():
    assert List[int](1, 2, 3, 4, 5) == List[int](1, 2, 3, 4, 5).fold_right(
        Nil(), lambda head, tail: Cons(head=head, tail=tail)
    )


def test_exercise39():
    assert 0 == Nil[int]().length
    assert 3 == List[int](1, 2, 3).length


def test_exercise310():
    assert 6 == List[int](1, 2, 3).fold_left(0, lambda a, b: a + b)


def test_exercise311():
    assert 6 == List[int](1, 2, 3).sum_left()
    assert 6.0 == List[int](1.0, 2.0, 3.0).product_left()
    assert 3 == List[int](1, 2, 3).length_left

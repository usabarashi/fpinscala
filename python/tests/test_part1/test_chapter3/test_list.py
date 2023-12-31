import pytest
from part1.chapter3.list import List, Nil, Cons


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


def test_exercise312():
    assert List[int](3, 2, 1) == List[int](1, 2, 3).reverse()


def test_exercise313():
    assert 6 == List[int](1, 2, 3).fold_left_from_right(0, lambda b, a: b + a)
    assert 6 == List[int](1, 2, 3).fold_right_from_left(0, lambda a, b: b + a)


def test_exercise314():
    assert List[int](1, 2, 3, 4, 5, 6) == List[int](1, 2, 3).append_right(List[int](4, 5, 6))


def test_exercise315():
    assert (
        List[int](1, 2, 3, 4, 5, 6) == List[int](List[int](1, 2, 3), List[int](4, 5, 6)).concat()
    )


def test_exercise316():
    assert List[int](2, 3, 4, 5, 6) == List[int](1, 2, 3, 4, 5).increment_each()


def test_exercise317():
    assert (
        List[str]("1.0", "2.0", "3.0", "4.0", "5.0")
        == List[float](1.0, 2.0, 3.0, 4.0, 5.0).double_to_string()
    )


def test_exercise318():
    assert List[int](2, 4, 6, 8, 10) == List[int](1, 2, 3, 4, 5).map(lambda x: x * 2)


def test_exercise319():
    assert List[int](1, 3, 5) == List[int](1, 2, 3, 4, 5).filter(lambda x: x % 2 != 0)


def test_exercise320():
    assert List[int](1, 1, 2, 2, 3, 3) == List[int](1, 2, 3).flat_map(lambda x: List[int](x, x))


def test_exercise321():
    assert List[int](1, 3) == List[int](1, 2, 3).filter_from_flat_map(lambda x: x % 2 != 0)


def test_exercise322():
    assert List[int](5, 7, 9) == List[int](1, 2, 3).add_pairwise(List[int](4, 5, 6))


def test_exercise323():
    assert List[int](5, 7, 9) == List[int](1, 2, 3).zip_with(
        List[int](4, 5, 6), lambda a, b: a + b
    )


def test_list_in_the_standard_library():
    assert List[int](1, 2, 3) == List[int](1, 2, 3, 4, 5).take(3)
    assert List[int](1, 2, 3) == List[int](1, 2, 3, 4, 5).take_while(lambda x: x <= 3)
    assert False == List[int](1, 2, 3, 4, 5).forall(lambda x: x <= 3)
    assert True == List[int](1, 2, 3, 4, 5).forall(lambda x: x <= 42)
    assert False == List[int](1, 2, 3, 4, 5).exists(lambda x: x == 42)
    assert True == List[int](1, 2, 3, 4, 5).exists(lambda x: x == 3)
    assert List[str]("", "a", "ab", "abc", "abcd", "abcde") == List[str]("a", "b", "c", "d", "e").scan_left("", lambda acc, x: acc + x)
    assert List[str]("abcde", "bcde", "cde", "de", "e", "") == List[str]("a", "b", "c", "d", "e").scan_right("", lambda x, acc: x + acc)

def test_exercise324():
    assert False == List[int](1, 2, 3, 4, 5).has_subsequence(List[int](4, 3, 2))
    assert True == List[int](1, 2, 3, 4, 5).has_subsequence(List[int](2, 3, 4))

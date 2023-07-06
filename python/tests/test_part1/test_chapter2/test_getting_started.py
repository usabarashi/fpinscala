from part1.chapter2.getting_started import curry, fib, is_sorted


def test_exercise21():
    assert 0 == fib(0)
    assert 1 == fib(1)
    assert 1 == fib(2)
    assert 2 == fib(3)
    assert 3 == fib(4)
    assert 5 == fib(5)
    assert 8 == fib(6)
    assert 13 == fib(7)
    assert 21 == fib(8)
    assert 34 == fib(9)
    try:
        fib(-42)
    except ValueError:
        assert True
    except Exception:
        assert False


def test_exercise22():
    assert True == is_sorted([1, 2, 3], lambda current, next: current > next)
    assert False == is_sorted([1, 2, 1], lambda current, next: current > next)
    assert True == is_sorted([3, 2, 1], lambda current, next: current < next)
    assert False == is_sorted([1, 2, 3], lambda current, next: current < next)

def test_exercise23():
    def func(a: int, b: int) -> int:
        return a + b

    assert func(a=42, b=42) == curry(func)(42)(42)

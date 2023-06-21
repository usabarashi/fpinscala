from part1.chapter2.getting_started import fib


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

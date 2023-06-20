from getting_started import MyProgram


def test_exersize21():
    assert 0 == MyProgram.fib(0)
    assert 1 == MyProgram.fib(1)
    assert 1 == MyProgram.fib(2)
    assert 2 == MyProgram.fib(3)
    assert 3 == MyProgram.fib(4)
    assert 5 == MyProgram.fib(5)
    assert 8 == MyProgram.fib(6)
    assert 13 == MyProgram.fib(7)
    assert 21 == MyProgram.fib(8)
    assert 34 == MyProgram.fib(9)
    try:
        MyProgram.fib(-42)
    except ValueError:
        assert True
    except Exception:
        assert False

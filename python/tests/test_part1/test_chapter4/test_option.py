from typing import cast

import pytest
from part1.chapter4.option import Option, Some, Void

def test_exercise41():
    assert Void is type(Void().map(lambda x: x + 1))
    map_some = Some(42).map(lambda x: x + 1)
    assert Some is type(map_some)
    assert 43 == map_some.value

    assert Void is type(Void().flat_map(lambda x: Some(42)))
    flat_map_some = Some(42).flat_map(lambda x: Some(x + 1))
    assert Some is type(flat_map_some)
    assert 43 == flat_map_some.value

    assert 0 == Void().get_or_else(lambda : 0)
    assert 42 == Some(42).get_or_else(lambda :0)

    or_else_void = Void().or_else(lambda : Some(42))
    assert Some is type(or_else_void)
    assert 42 == cast(Some[int], or_else_void).value
    or_else_some = Some(42).or_else(lambda : Some(0))
    assert Some is type(or_else_some)
    assert 42 == cast(Some[int], or_else_void).value

    assert Void is type(Void().filter(lambda x: x == 42))
    filter_some = Some(42).filter(lambda x: x ==42)
    assert Some is type(filter_some)
    assert 42 == cast(Some[int], filter_some).value

def test_exercise42():
    result = Option.variance([1.0, 2.0, 3.0, 4.0, 5.0])
    assert Some is type(result)
    assert 2.0 == result.value

def test_exercise43():
    result_none = Some[int](42).map2(Void[int](), lambda a, b: a + b)
    assert Void is type(result_none)

    result_some = Some[int](42).map2(Some[int](42), lambda a, b: a + b)
    assert Some is type(result_some)
    assert 84 == cast(Some[int], result_some).value

def test_exercise44():
    result_none = Option.sequence([Some(42), Void()])
    assert Void is type(result_none)

    result_some = Option.sequence([Some(42), Some(42)])
    assert Some is type(result_some)
    assert [42, 42] == result_some.value

def test_exercise45():
    result_none = Option.sequence_from_traverse([Some(42), Void()])
    assert Void is type(result_none)

    result_some = Option.sequence_from_traverse([Some(42), Some(42)])
    assert Some is type(result_some)
    assert [42, 42] == result_some.value

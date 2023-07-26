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

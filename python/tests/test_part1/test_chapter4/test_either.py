import pytest

from part1.chapter4.either import Either, Left, Right

def test_exercise46():
    assert Left is type(Left[str, int]("error").map(lambda x: x))
    map_right = Right[str, int](42).map(lambda x: x)
    assert Right is type(map_right)
    assert 42 == map_right.value

    assert Left is type(Left[str, int]("error").flat_map(lambda x: Right[str, int](42)))
    flat_map_right = Right[str, int](42).flat_map(lambda x: Right[str, int](x))
    assert Right is type(flat_map_right)
    assert 42 == flat_map_right.value

    or_else_left = Left[str, int]("error").or_else(lambda : Left[str, int]("error"))
    assert Left is type(or_else_left)
    assert "error" == or_else_left.value
    or_else_right = Right[str, int](42).or_else(lambda : Left[str, int]("error"))
    assert Right is type(or_else_right)
    assert 42 == or_else_right.value

    assert Left is type(Left[str, int]("error").map2(Right[str, int](42), lambda a, b: a + b))
    map2_right = Right[str, int](42).map2(Right[str, int](42), lambda a, b: a + b)
    assert Right is type(map2_right)
    assert 84 == map2_right.value

def test_exercise47():
    sequence_left = Either.sequence([Right(42), Left("error")])
    assert Left is type(sequence_left)
    sequence_right = Either.sequence([Right(42), Right(42)])
    assert Right is type(sequence_right)
    assert [42, 42] == sequence_right.value

    traverse_left = Either.traverse([Right(42), Left("error")], lambda x: x)
    assert Left is type(traverse_left)
    traverse_right = Either.traverse([Right(42), Right(42)], lambda x: x)
    assert Right is type(traverse_right)
    assert [42, 42] == traverse_right.value

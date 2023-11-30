from __future__ import annotations

from collections.abc import Generator
from functools import lru_cache
from typing import Callable, Generic, TypeVar, TypeAlias, cast, Iterable

from src.part1.chapter3.list import List, Nil
from src.part1.chapter4.option import Option, Some, Void

A = TypeVar("A")
Thunk: TypeAlias = Callable[[], A]


class LazyList(Generic[A], Iterable):
    def __init__(self):
        pass

    @staticmethod
    def cons(head: Thunk[A], tail: Thunk[LazyList[A]]) -> LazyList[A]:
        return Cons(head=lru_cache(maxsize=None)(head), tail=lru_cache(maxsize=None)(tail))

    @staticmethod
    def empty() -> LazyList[A]:
        return Empty()

    @staticmethod
    def apply(*as_: A) -> LazyList[A]:
        """
        deprecated

        評価済みの可変長引数を遅延評価に再構築することに意義はない.
        map関数のように未評価関数を挟むことで遅延評価の意義が生まれる.
        """
        if len(as_) == 0:
            return LazyList.empty()
        else:
            return LazyList.cons(lambda: as_[0], lambda: LazyList.apply(*as_[1:]))

    def headOption(self) -> Option[A]:
        raise NotImplementedError(self)

    def toList(self) -> List[A]:
        def go(lazy_list: LazyList[A], accumulator: List[A]) -> List[A]:
            match cast(SubType[A], lazy_list).pattern:
                case Empty():
                    return accumulator.reverse()
                case Cons(head, tail):
                    return go(tail(), List[A](head()).append(accumulator))
                case _:
                    raise AssertionError("Unreachable code", lazy_list)

        return go(self, Nil[A]())

    @property
    def pattern(self) -> SubType:
        raise NotImplementedError(self)


class Empty(LazyList[A]):
    def __init__(self):
        super().__init__()

    def __iter__(self) -> Generator[A, None, None]:
        raise GeneratorExit(self)

    def headOption(self) -> Option[A]:
        return cast(Option[A], Void[A]())

    @property
    def pattern(self) -> SubType:
        return self


class Cons(LazyList[A]):
    def __init__(self, head: Thunk[A], tail: Thunk[LazyList[A]]):
        super().__init__()
        self.head = head
        self.tail = tail

    def __iter__(self) -> Generator[A, None, None]:
        yield self.head()
        yield from self.tail()

    def headOption(self) -> Option[A]:
        return cast(Option[A], Some[A](self.head()))

    @property
    def pattern(self) -> SubType:
        return self


SubType: TypeAlias = Empty[A] | Cons[A]

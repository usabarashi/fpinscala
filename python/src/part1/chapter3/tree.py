"""Tree."""
from __future__ import annotations

from dataclasses import dataclass
from typing import Generic, TypeVar, TypeAlias, Callable

A = TypeVar("A")
B = TypeVar("B")


class Tree(Generic[A]):
    """Tree"""

    @property
    def pattern(self) -> SubType[A]:
        raise NotImplementedError()

    def size(self) -> int:
        match self.pattern:
            case Leaf():
                return 1
            case Branch(left, right):
                return 1 + left.size() + right.size()

    def maximum(self: Tree[int]) -> int:
        match self.pattern:
            case Leaf(value):
                return value
            case Branch(left, right):
                return max(left.maximum(), right.maximum())

    def depth(self) -> int:
        match self.pattern:
            case Leaf():
                return 0
            case Branch(left, right):
                return 1 + max(left.depth(), right.depth())

    def map(self, f: Callable[[A], B]) -> Tree[B]:
        match self.pattern:
            case Leaf(value):
                return Leaf(f(value))
            case Branch(left, right):
                return Branch(left.map(f), right.map(f))

    def fold(self, f: Callable[[A], B], g: Callable[[B, B], B]) -> B:
        match self.pattern:
            case Leaf(value):
                return f(value)
            case Branch(left, right):
                return g(left.fold(f, g), right.fold(f, g))

    def size_via_fold(self) -> int:
        return self.fold(lambda _: 1, lambda left, right: 1 + left + right)

    def depth_via_fold(self) -> int:
        return self.fold(lambda _: 0, lambda left, right: 1 + max(left, right))

    def map_via_fold(self, f: Callable[[A], B]) -> Tree[B]:
        return self.fold(lambda value: Leaf(f(value)), lambda left, right: Branch(left, right))


@dataclass(frozen=True)
class Leaf(Tree[A]):
    """Leaf"""

    value: A

    @property
    def pattern(self) -> SubType[A]:
        return self


@dataclass(frozen=True)
class Branch(Tree[A]):
    """Branch"""

    left: Tree[A]
    right: Tree[A]

    @property
    def pattern(self) -> SubType[A]:
        return self


SubType: TypeAlias = Leaf[A] | Branch[A]

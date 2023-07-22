"""Tree."""
from __future__ import annotations

from dataclasses import dataclass
from typing import Generic, TypeVar, TypeAlias

A = TypeVar("A")


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

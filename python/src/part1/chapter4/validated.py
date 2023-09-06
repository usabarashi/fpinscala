from __future__ import annotations

from dataclasses import dataclass
from functools import reduce
from typing import Generic, TypeVar, final, TypeAlias, Callable, cast

from part1.chapter4.either import Either, Left, Right

E = TypeVar("E", covariant=True)
A = TypeVar("A", covariant=True)
B = TypeVar("B", covariant=True)
C = TypeVar("C", covariant=True)


@dataclass(frozen=True)
class Validated(Generic[E, A]):
    value: list[E] | A

    def map(self, f: Callable[[A], B]) -> Validated[E, B]:
        match self.pattern:
            case Invalid(_):
                return cast(Validated[E, B], self)
            case Valid(value):
                return cast(Validated[E, B], Valid(f(value)))

    def map2(self, other: Validated[E, B], f: Callable[[A, B], C]) -> Validated[E, C]:
        match (self.pattern, other.pattern):
            case (Valid(aa), Valid(bb)):
                return cast(Validated[E, C], Valid(f(aa, bb)))
            case (Invalid(es), Valid(_)):
                return cast(Validated[E, C], Invalid(es))
            case (Valid(_), Invalid(es)):
                return cast(Validated[E, C], Invalid(es))
            case (Invalid(es1), Invalid(es2)):
                return cast(Validated[E, C], Invalid(es1 + es2))
            case _:
                raise ValueError(self, other)

    @staticmethod
    def from_either(either: Either[list[E], A]) -> Validated[E, A]:
        match either.pattern:
            case Left(errors):
                return cast(Validated[E, A], Invalid(errors))
            case Right(value):
                return cast(Validated[E, A], Valid(value))

    def to_either(self) -> Either[list[E], A]:
        match self.pattern:
            case Invalid(error):
                return cast(Either[list[E], A], Left(error))
            case Valid(value):
                return cast(Either[list[E], A], Right(value))

    @staticmethod
    def traverse(xs: list[A], f: Callable[[A], Validated[E, B]]) -> Validated[E, list[B]]:
        return reduce(lambda acc, a: f(a).map2(acc, lambda b, bs: [b] + bs), xs, Valid[E, B]([]))

    @staticmethod
    def sequence(vs: list[Validated[E, A]]) -> Validated[E, list[A]]:
        return Validated.traverse(vs, lambda x: x)

    @property
    def pattern(self) -> SubType[E, A]:
        return cast(SubType[E, A], self)


@final
class Invalid(Validated[E, A]):
    value: list[E]


@final
class Valid(Validated[E, A]):
    value: A


SubType: TypeAlias = Invalid[E, A] | Valid[E, A]

from __future__ import annotations

from dataclasses import dataclass
from typing import Generic, TypeVar, TypeAlias, Callable, cast

Lp = TypeVar("Lp", covariant=True)
LLp = TypeVar("LLp", covariant=True)
Rp = TypeVar("Rp", covariant=True)
RRp = TypeVar("RRp", covariant=True)
RRRp = TypeVar("RRRp", covariant=True)


class Either(Generic[Lp, Rp]):
    def map(self, f: Callable[[Rp], RRp]) -> Either[RRp]:
        match self.pattern:
            case Left():
                return cast(Left[Lp, Rp], self)
            case Right(value):
                return Right[Lp, RRp](f(value))

    def flat_map(self, f: Callable[[Rp], Either[Lp, RRp]]) -> Either[RRp]:
        match self.pattern:
            case Left():
                return cast(Left[Lp, RRp], self)
            case Right(value):
                return f(value)

    def or_else(self, ob: Callable[[], Either[LLp, Rp]]) -> Either[LLp, Rp]:
        match self.pattern:
            case Left():
                return ob()
            case Right():
                return cast(Right[LLp, Rp], self)

    def map2(self, other: Either[Lp, RRp], f: Callable[[Rp, RRp], RRRp]) -> Either[Lp, RRRp]:
        match self.pattern, other.pattern:
            case (Left(), _) | (_, Left()):
                return cast(Either[Lp, RRRp], self)
            case Right(a), Right(b):
                return Right[Lp, RRRp](f(a, b))

    @property
    def pattern(self) -> SubType:
        ...


@dataclass(frozen=True)
class Left(Either[Lp, Rp]):
    value: Lp

    @property
    def pattern(self) -> SubType:
        return self


@dataclass(frozen=True)
class Right(Either[Lp, Rp]):
    value: Rp

    @property
    def pattern(self) -> SubType:
        return self


SubType: TypeAlias = Left | Right

from __future__ import annotations

from dataclasses import dataclass
from typing import Generic, TypeVar, TypeAlias, Callable, cast

Ap = TypeVar("Ap", covariant=True)
Bp = TypeVar("Bp", covariant=True)


class Option(Generic[Ap]):
    def map(self, f: Callable[[Ap], Bp]) -> Option[Bp]:
        match self.pattern:
            case Void():
                return cast(Option[Bp], Void())
            case Some(value):
                return Some(f(value))

    def flat_map(self, f: Callable[[Ap], Option[Bp]]) -> Option[Bp]:
        return self.map(f).get_or_else(lambda: cast(Option[Bp], Void()))

    def get_or_else(self, default: Callable[[], Bp]) -> Bp:
        match self.pattern:
            case Void():
                return default()
            case Some(value):
                return value

    def or_else(self, ob: Callable[[], Option[Bp]]) -> Option[Bp]:
        return self.map(lambda x: Some(x)).get_or_else(ob)

    def filter(self, f: Callable[[Ap], bool]) -> Option[Ap]:
        return self.flat_map(lambda x: Some(x) if (x) else cast(Option[Ap], Void()))

    @property
    def pattern(self) -> SubType:
        ...


class Void(Option[object]):
    def __new__(cls) -> Void:
        if not hasattr(cls, "_singleton"):
            cls._singleton = super(Void, cls).__new__(cls)
        return cls._singleton

    @property
    def pattern(self) -> SubType:
        return self


@dataclass(frozen=True)
class Some(Option[Ap]):
    value: Ap

    @property
    def pattern(self) -> SubType:
        return self


SubType: TypeAlias = Void | Some

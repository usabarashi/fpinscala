from __future__ import annotations

from dataclasses import dataclass
from typing import Generic, TypeVar, TypeAlias, Callable

Ap = TypeVar("Ap", covariant=True)
Bp = TypeVar("Bp", covariant=True)


class Option(Generic[Ap]):
    def map(self, f: Callable[[Ap], Bp]) -> Option[Bp]:
        match self.pattern:
            case Void():
                return Void[Bp]()
            case Some(value):
                return Some(f(value))

    def flat_map(self, f: Callable[[Ap], Option[Bp]]) -> Option[Bp]:
        return self.map(f).get_or_else(lambda: Void[Bp]())

    def get_or_else(self, default: Callable[[], Bp]) -> Bp:
        match self.pattern:
            case Void():
                return default()
            case Some(value):
                return value

    def or_else(self, ob: Callable[[], Option[Bp]]) -> Option[Bp]:
        return self.map(lambda x: Some(x)).get_or_else(ob)

    def filter(self, f: Callable[[Ap], bool]) -> Option[Ap]:
        return self.flat_map(lambda x: Some(x) if (x) else Void[Ap]())

    @staticmethod
    def mean(xs: list[float]) -> Option[float]:
        match xs:
            case []:
                return Void[float]()
            case _:
                return Some[float](sum(xs) / len(xs))

    @staticmethod
    def variance(xs: list[float]) -> Option[float]:
        return Option.mean(xs).flat_map(lambda m: Option.mean([pow(x - m, 2) for x in xs]))

    @property
    def pattern(self) -> SubType:
        ...


class Void(Option[Ap]):
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

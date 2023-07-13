"""Functional data structure."""
from __future__ import annotations

from typing import Generic, TypeVar, TypeAlias

Tp = TypeVar("Tp", covariant=True)


class List(Generic[Tp]):
    """List"""

    @staticmethod
    def apply(*args: * tuple[Tp, ...]) -> List[Tp]:
        match args:
            case ():
                return Nil[Tp]()
            case (x,):
                return Cons[Tp](head=x, tail=Nil[Tp]())
            case (x, *xs):
                return Cons[Tp](head=x, tail=List[Tp].apply(*xs))

    def __new__(cls, *args: * tuple[Tp, ...]) -> List[Tp]:
        return List.apply(*args)

    def sum(self) -> int:
        match self.pattern:
            case Nil():
                return 0
            case Cons(head=x, tail=xs):
                return x + xs.sum()

    @property
    def pattern(self) -> SubType[Tp]:
        raise NotImplementedError()


class Nil(List[Tp]):
    """Nil"""

    def __new__(cls) -> Nil[Tp]:
        if not hasattr(cls, "_singleton"):
            cls._singleton = object.__new__(cls)
        return cls._singleton

    @property
    def pattern(self) -> SubType[Tp]:
        return self


class Cons(List[Tp]):
    """Constructure"""

    __match_args__ = ("head", "tail")

    def __new__(cls, *, head: Tp, tail: List[Tp]) -> Cons[Tp]:
        instance = object.__new__(cls)
        instance.head = head
        instance.tail = tail
        return instance

    @property
    def pattern(self) -> SubType[Tp]:
        return self


SubType: TypeAlias = Nil[Tp] | Cons[Tp]

"""Functional data structure."""
from __future__ import annotations

from typing import Callable, Generic, TypeVar, TypeAlias

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

    def __eq__(self, other: List[Tp]) -> bool:
        match (self, other):
            case (Nil(), Nil()):
                return True
            case (Nil(), Cons()) | (Cons(), Nil()):
                return False
            case (
                Cons(head=left_head, tail=left_tail),
                Cons(head=right_head, tail=right_tail),
            ):
                return left_head == right_head and left_tail == right_tail

    def sum(self: List[int]) -> int:
        match self.pattern:
            case Nil():
                return 0
            case Cons(head=x, tail=xs):
                return x + xs.sum()

    def set_head(self, a: Tp) -> List[Tp]:
        match self.pattern:
            case Nil():
                raise Exception(self)
            case Cons(head=x, tail=xs):
                return Cons(head=a, tail=xs)

    def drop(self, n: int) -> List[Tp]:
        match self.pattern:
            case Cons(_, tail) if 0 < n:
                return tail.drop(n - 1)
            case _:
                return self

    def drop_while(self, f: Callable[[Tp], bool]) -> List[Tp]:
        match self.pattern:
            case Cons(head, tail) if f(head):
                return tail.drop_while(f)
            case _:
                return self

    def init(self) -> List[Tp]:
        match self.pattern:
            case Nil():
                raise EOFError(self)
            case Cons(_, Nil()):
                return Nil[Tp]()
            case Cons(head, tail):
                return Cons(head=head, tail=tail.init())

    @property
    def tail(self) -> List[Tp]:
        raise NotImplementedError(self)

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
    def tail(self) -> List[Tp]:
        raise EOFError(self)

    @property
    def pattern(self) -> SubType[Tp]:
        return self


class Cons(List[Tp]):
    """Constructure"""

    __match_args__ = ("head", "tail")

    def __new__(cls, *, head: Tp, tail: List[Tp]) -> Cons[Tp]:
        instance = object.__new__(cls)
        instance.head = head
        instance._tail = tail
        return instance

    @property
    def tail(self) -> List[Tp]:
        return self._tail

    @property
    def pattern(self) -> SubType[Tp]:
        return self


SubType: TypeAlias = Nil[Tp] | Cons[Tp]

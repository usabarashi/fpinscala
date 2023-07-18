"""Functional data structure."""
from __future__ import annotations

from collections.abc import Generator, Iterable
from functools import reduce
from typing import Callable, Generic, TypeVar, TypeAlias

Tp = TypeVar("Tp", covariant=True)
Bp = TypeVar("Bp", covariant=True)


class List(Generic[Tp], Iterable):
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

    def __iter__(self) -> Generator[Tp, None, None]:
        mut_state = self
        while isinstance(mut_state, Cons):
            yield mut_state.head
            mut_state = mut_state.tail

    def sum(self: List[int]) -> int:
        match self.pattern:
            case Nil():
                return 0
            case Cons(head=x, tail=xs):
                return x + xs.sum()

    def product(self: List[float]) -> float:
        match self.pattern:
            case Nil():
                return 1.0
            case Cons(head=x, tail=xs):
                return x * xs.product()

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

    def fold_right(self, accumulator: Bp, f: Callable[[Tp, Bp], Bp]) -> Bp:
        match self.pattern:
            case Nil():
                return accumulator
            case Cons(head, tail):
                return f(head, tail.fold_right(accumulator, f))

    def fold_left(self, accumulator: Bp, f: Callable[[Bp, Tp], Bp]) -> Bp:
        return reduce(f, self, accumulator)

    def sum_left(self) -> int:
        return reduce(lambda accumulator, head: accumulator + head, self, 0)

    def product_left(self) -> float:
        return reduce(lambda accumulator, head: accumulator * head, self, 1.0)

    def reverse(self) -> List[Tp]:
        return self.fold_left(
            Nil[Tp](), lambda accumulator, head: Cons[Tp](head=head, tail=accumulator)
        )

    def fold_left_from_right(self, accumulator: Bp, f: Callable[[Bp, Tp], Bp]) -> Bp:
        return self.fold_right(
            accumulator, lambda head, accumulator_: f(accumulator_, head)
        )

    def fold_right_from_left(self, accumulator: Bp, f: Callable[[Tp, Bp], Bp]) -> Bp:
        return self.fold_left(
            accumulator, lambda accumulator_, head: f(head, accumulator_)
        )

    def append_right(self, a2: List[Tp]) -> List[Tp]:
        return self.fold_right(
            a2, lambda head, accumulator: Cons[Tp](head=head, tail=accumulator)
        )

    @property
    def length_left(self) -> int:
        return reduce(lambda accumulator, _: accumulator + 1, self, 0)

    @property
    def length(self) -> int:
        return self.fold_right(0, lambda _, b: b + 1)

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

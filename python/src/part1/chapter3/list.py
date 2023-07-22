"""List."""
from __future__ import annotations

from collections.abc import Generator, Iterable
from functools import reduce
from typing import Callable, TypeVar, TypeAlias, cast

A = TypeVar("A")
B = TypeVar("B")
Bp = TypeVar("Bp", covariant=True)
Cm = TypeVar("Cm", contravariant=True)


class List(Iterable[A]):
    """List"""

    @staticmethod
    def apply(*args: * tuple[A, ...]) -> List[A]:
        match args:
            case ():
                return Nil[A]()
            case (x,):
                return Cons[A](head=x, tail=Nil[A]())
            case (x, *xs):
                return Cons[A](head=x, tail=List[A].apply(*xs))
            case _:
                raise ValueError(args)

    def __new__(cls, *args: * tuple[A, ...]) -> List[A]:
        return List.apply(*args)

    def __eq__(self, other: List[A]) -> bool:  # type: ignore
        match (self.pattern, other.pattern):
            case (Nil(), Nil()):
                return True
            case (Nil(), Cons()) | (Cons(), Nil()):
                return False
            case (Cons() as self_cons, Cons() as other_cons):
                return self_cons.head == other_cons.head and self_cons.tail == other_cons.tail
            case _:
                return False

    def __iter__(self) -> Generator[A, None, None]:
        mut_state = self
        while isinstance(mut_state, Cons):
            yield mut_state.head
            mut_state = mut_state.tail

    def sum(self: List[int]) -> int:
        match self.pattern:
            case Nil():
                return 0
            case Cons(head, tail):
                return head + tail.sum()

    def product(self: List[float]) -> float:
        match self.pattern:
            case Nil():
                return 1.0
            case Cons(head, tail):
                return head * tail.product()

    def set_head(self, head: A) -> List[A]:
        match self.pattern:
            case Nil():
                raise Exception(self)
            case Cons(_, tail):
                return Cons(head=head, tail=tail)

    def drop(self, n: int) -> List[A]:
        match self.pattern:
            case Cons() if 0 < n:
                return self.tail.drop(n - 1)
            case _:
                return self

    def drop_while(self, f: Callable[[A], bool]) -> List[A]:
        match self.pattern:
            case Cons(head, tail) if f(head):
                return tail.drop_while(f)
            case _:
                return self

    def append(self, other: List[A]) -> List[A]:
        match self.pattern:
            case Nil():
                return other
            case Cons(head, tail):
                return Cons(head=head, tail=tail.append(other))

    def init(self) -> List[A]:
        match self.pattern:
            case Nil():
                raise EOFError(self)
            case Cons(_, Nil()):
                return Nil[A]()
            case Cons(head, tail):
                return Cons(head=head, tail=tail.init())

    def fold_right(self, accumulator: B, f: Callable[[A, B], B]) -> B:
        match self.pattern:
            case Nil():
                return accumulator
            case Cons(head, tail):
                return f(head, tail.fold_right(accumulator, f))

    def fold_left(self, accumulator: B, f: Callable[[B, A], B]) -> B:
        return reduce(f, self, accumulator)

    def sum_left(self: List[int]) -> int:
        return reduce(lambda accumulator, head: accumulator + head, self, 0)

    def product_left(self: List[float]) -> float:
        return reduce(lambda accumulator, head: accumulator * head, self, 1.0)

    def reverse(self) -> List[A]:
        return self.fold_left(
            Nil[A](), lambda accumulator, head: Cons[A](head=head, tail=accumulator)
        )

    def fold_left_from_right(self, accumulator: B, f: Callable[[B, A], B]) -> B:
        return self.fold_right(accumulator, lambda head, accumulator_: f(accumulator_, head))

    def fold_right_from_left(self, accumulator: B, f: Callable[[A, B], B]) -> B:
        return self.fold_left(accumulator, lambda accumulator_, head: f(head, accumulator_))

    def append_right(self, other: List[A]) -> List[A]:
        return self.fold_right(
            other, lambda head, accumulator: Cons[A](head=head, tail=accumulator)
        )

    def concat(self: List[List[A]]) -> List[A]:
        return self.fold_right(Nil[A](), List[A].append)  # type: ignore

    def increment_each(self: List[int]) -> List[int]:
        return self.fold_right(
            Nil[int](),
            lambda head, accumulator: Cons[int](head=head + 1, tail=accumulator),
        )

    def double_to_string(self: List[float]) -> List[str]:
        return self.fold_right(
            Nil[str](),
            lambda head, accumulator: Cons[str](head=str(head), tail=accumulator),
        )

    def map(self, f: Callable[[A], Bp]) -> List[Bp]:
        return self.fold_right(
            Nil[Bp](), lambda head, accumulator: Cons(head=f(head), tail=accumulator)
        )

    def filter(self, f: Callable[[A], bool]) -> List[A]:
        return self.fold_right(
            Nil[A](),
            lambda head, accumulator: Cons(head=head, tail=accumulator)
            if f(head)
            else accumulator,
        )

    def flat_map(self, f: Callable[[A], List[B]]) -> List[B]:
        return self.map(f).concat()  # type: ignore

    def filter_from_flat_map(self, f: Callable[[A], bool]) -> List[A]:
        return self.flat_map(lambda x: List[A](x) if f(x) else Nil[A]())

    def add_pairwise(self: List[int], other: List[int]) -> List[int]:
        match (self.pattern, other.pattern):
            case (Nil(), _) | (_, Nil()):
                return Nil[int]()
            case (Cons() as self_cons, Cons() as other_cons):
                return Cons(
                    head=self_cons.head + other_cons.head,
                    tail=self_cons.tail.add_pairwise(other_cons.tail),
                )

    def zip_with(self, other: List[Bp], f: Callable[[A, Bp], Cm]) -> List[Cm]:
        match (self.pattern, other.pattern):
            case (Nil(), _) | (_, Nil()):
                return Nil[Cm]()
            case (Cons() as self_cons, Cons() as other_cons):
                return Cons(
                    head=f(self_cons.head, other_cons.head),
                    tail=self_cons.tail.zip_with(other_cons.tail, f),
                )

    def take(self, n: int) -> List[A]:
        match self.pattern:
            case Nil():
                return self
            case Cons() if n <= 0:
                return Nil[A]()
            case Cons(head, tail):
                return Cons[A](head=head, tail=tail.take(n - 1))

    def take_while(self, f: Callable[[A], bool]) -> List[A]:
        match self.pattern:
            case Nil():
                return self
            case Cons(head, _) if not f(head):
                return Nil[A]()
            case Cons(head, tail):
                return Cons(head=head, tail=tail.take_while(f))

    def forall(self, f: Callable[[A], bool]) -> bool:
        return self.fold_left(True, lambda accumulator, head: accumulator and f(head))

    def exists(self, f: Callable[[A], bool]) -> bool:
        return self.fold_left(False, lambda accumulator, head: accumulator or f(head))

    def scan_left(self, accumulator: B, f: Callable[[B, A], B]) -> List[B]:
        match self.pattern:
            case Nil():
                return List[B](accumulator)
            case Cons(head, tail):
                return Cons(head=accumulator, tail=tail.scan_left(f(accumulator, head), f))

    def scan_right(self, accumulator: B, f: Callable[[A, B], B]) -> List[B]:
        match self.pattern:
            case Nil():
                return List[B](accumulator)
            case Cons(head, tail):
                new_tail = cast(Cons[B], tail.scan_right(accumulator, f))
                new_head = f(head, new_tail.head)
                return Cons(head=new_head, tail=new_tail)

    def start_with(self, prefix: List[A]) -> bool:
        match (self.pattern, prefix.pattern):
            case (_, Nil()):
                return True
            case (Cons() as scons, Cons() as pcons) if scons.head == pcons.head:
                return scons.tail.start_with(pcons.tail)
            case _:
                return False

    def has_subsequence(self, sub: List[A]) -> bool:
        match self.pattern:
            case Nil():
                return sub == Nil[A]()
            case _ if self.start_with(sub):
                return True
            case Cons(_, tail):
                return tail.has_subsequence(sub)

    @property
    def length_left(self) -> int:
        return reduce(lambda accumulator, _: accumulator + 1, self, 0)

    @property
    def length(self) -> int:
        return self.fold_right(0, lambda _, b: b + 1)

    @property
    def tail(self) -> List[A]:
        raise NotImplementedError(self)

    @property
    def pattern(self) -> SubType[A]:
        raise NotImplementedError()


class Nil(List[A]):
    """Nil"""

    def __new__(cls) -> Nil[A]:
        if not hasattr(cls, "_singleton"):
            cls._singleton = object.__new__(cls)
        return cls._singleton

    @property
    def tail(self) -> List[A]:
        raise EOFError(self)

    @property
    def pattern(self) -> SubType[A]:
        return self


class Cons(List[A]):
    """Constructor"""

    _head: A
    _tail: List[A]

    __match_args__ = ("head", "tail")

    def __new__(cls, *, head: A, tail: List[A]) -> Cons[A]:
        instance = object.__new__(cls)
        instance._head = head
        instance._tail = tail
        return instance

    @property
    def head(self) -> A:
        return self._head

    @property
    def tail(self) -> List[A]:
        return self._tail

    @property
    def pattern(self) -> SubType[A]:
        return self


SubType: TypeAlias = Nil[A] | Cons[A]

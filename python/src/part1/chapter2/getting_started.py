from typing import Callable, TypeVar, cast, overload

A = TypeVar("A")
B = TypeVar("B")
C = TypeVar("C")


class MyProgram:
    @staticmethod
    def abs(n: int) -> int:
        return (-1 * n) if n < 0 else n

    @classmethod
    def _format_abs(cls, x: int) -> str:
        return f"The absolute value of {x} is {cls.abs(x)}"

    @classmethod
    def print_abs(cls):
        print(cls._format_abs(-42))

    @staticmethod
    def factorial(n: int) -> int:
        if n < 0:
            raise ValueError(n)
        mut_result = 1
        for i in range(1, n + 1):
            mut_result *= i
        return mut_result

    @classmethod
    def _format_factorial(cls, n: int) -> str:
        return f"The factorial of {n} is {cls.factorial(n)}."

    @classmethod
    def print_abs_and_factorial(cls):
        print(cls._format_abs(-42))
        print(cls._format_factorial(7))

    @classmethod
    def format_result(cls, name: str, n: int, f: Callable[[int], int]) -> str:
        return f"The {name} of {n} is {f(n)}."

    @overload
    @staticmethod
    def find_first(as_: list[str], key: str) -> int:
        ...

    @overload
    @staticmethod
    def find_first(as_: list[A], p: Callable[[A], bool]) -> int:
        ...

    @staticmethod
    def find_first(as_: list[str | A], key_p: str | Callable[[A], bool]) -> int:  # type: ignore
        if isinstance(key_p, str):
            for index, element in enumerate(cast(list[str], as_)):
                if element == key_p:
                    return index
            return -1
        elif callable(key_p):
            for index, element in enumerate(cast(list[A], as_)):
                if key_p(element):
                    return index
            return -1
        else:
            return -1


def fib(n: int) -> int:
    mut_current, mut_next = 0, 1
    for _ in range(n):
        mut_current, mut_next = mut_next, mut_current + mut_next
    return mut_current


def is_sorted(as_: list[A], gt: Callable[[A, A], bool]) -> bool:
    for current, next in zip(as_, as_[1:]):
        if gt(current, next):
            return False
    return True


def curry(f: Callable[[A, B], C]) -> Callable[[A], Callable[[B], C]]:
    return lambda a: lambda b: f(a, b)


def uncurry(f: Callable[[A], Callable[[B], C]]) -> Callable[[A, B], C]:
    return lambda a, b: f(a)(b)


def compose(f: Callable[[B], C], g: Callable[[A], B]) -> Callable[[A], C]:
    return lambda a: f(g(a))

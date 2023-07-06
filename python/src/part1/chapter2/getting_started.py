from typing import Callable, TypeVar, overload

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
    def _format_factorial(cls, n: int) -> int:
        return f"The factorial of {n} is {cls.factorial(n)}."

    @classmethod
    def print_abs_and_factorial(cls):
        print(cls._format_abs(-42))
        print(cls._format_factorial(7))

    @classmethod
    def format_result(name: str, n: int, f: Callable[[int], int]) -> str:
        return f"The {name} of {n} is {f(n)}."

    @overload
    @staticmethod
    def find_first(as_: list[str], key: str) -> int:
        raise NotImplementedError()

    @overload
    @staticmethod
    def find_first(as_: list[A], p: Callable[[A], bool]) -> int:
        raise NotImplementedError()

    @staticmethod
    def find_first(as_: list[str | A], key_p: str | Callable[[A], bool]) -> int:
        if isinstance(key_p, str):
            for index, element in enumerate(as_):
                if element == key_p:
                    return index
            return -1
        elif callable(key_p):
            for index, element in enumerate(as_):
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
    def partial_application_1(arg1: A) -> Callable[[B], C]:
        def partial_application_2(arg2: B) -> C:
            return f(arg1, arg2)

        return partial_application_2

    return partial_application_1


def uncurry(f: Callable[[A], Callable[[B], C]]) -> Callable[[A, B], C]:
    def application(arg1: A, arg2: B) -> C:
        return f(arg1)(arg2)

    return application

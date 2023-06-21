from typing import Callable


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


@staticmethod
def fib(n: int) -> int:
    mut_current, mut_next = 0, 1
    for _ in range(n):
        mut_current, mut_next = mut_next, mut_current + mut_next
    return mut_current

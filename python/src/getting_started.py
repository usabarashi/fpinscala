class MyProgram:
    @staticmethod
    def abs(n: int) -> int:
        return (-1 * n) if n < 0 else n

    @classmethod
    def _from_abs(cls, x: int) -> str:
        return f"The absolute value of {x} is {cls.abs(x)}"

    @classmethod
    def print_abs(cls):
        print(cls._from_abs(-42))

    @staticmethod
    def factorial(n: int) -> int:
        if n < 0:
            raise ValueError(n)
        mut_result = 1
        for i in range(1, n + 1):
            mut_result *= i
        return mut_result

    @staticmethod
    def fib(n: int) -> int:
        # match n:
        #     case 0:
        #         return 0
        #     case 1:
        #         return 1
        #     case _ if 1 < n:
        #         return fib(n - 2) + fib(n - 1)
        #     case _:
        #         raise ValueError(n)

        mut_current, mut_next = 0, 1
        for _ in range(n):
            mut_current, mut_next = mut_next, mut_current + mut_next
        return mut_current

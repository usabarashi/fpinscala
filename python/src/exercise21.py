def fib(n: int) -> int:
    match n:
        case 0:
            return 0
        case 1:
            return 1
        case _ if 1 < n:
            return fib(n-2) + fib(n-1)
        case _:
            raise ValueError(n)

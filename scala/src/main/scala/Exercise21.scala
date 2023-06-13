def fib(n: Int): Int =
  n match
    case 0 => 0
    case 1 => 1
    case _ if 1 < n => fib(n-2) + fib(n-1)
    case _ => throw new IllegalArgumentException(s"$n")

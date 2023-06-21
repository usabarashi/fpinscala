import annotation.tailrec

object MyProgram:
  def abs(n: Int): Int =
    if n < 0 then -n else n

  private def formatAbs(x: Int): String =
    val msg = "The absolute value of %d is %d"
    msg.format(x, abs(x))

  @main def printAbs: Unit =
    println(formatAbs(-42))

  def factorial(n: Int): Int =
    @tailrec
    def go(n: Int, acc: Int): Int =
      if n <= 0 then acc
      else go(n - 1, n * acc)

    go(n, 1)

  private def formatFactorial(n: Int): String =
    val msg = "The factorial of %d is %d."
    msg.format(n, factorial(n))

  def printAbsAndFactorial: Unit =
    println(formatAbs(-42))
    println(formatFactorial(7))

  def formatResult(name: String, n: Int, f: Int => Int): String =
    val msg = "The %s of %d is %d."
    msg.format(name, n, f(n))

def fib(n: Int): Int =
  if (n < 0) throw new IllegalArgumentException(s"$n")
  @tailrec
  def go(number: Int, current: Int, next: Int): Int =
    // n match
    //   case 0 => 0
    //   case 1 => 1
    //   case _ => fib(n-2) + fib(n-1)
    if number <= 0 then current
    else go(number=number - 1, current=next, next=current + next)
  go(number=n, current=0, next=1)

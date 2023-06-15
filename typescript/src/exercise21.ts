export const fib = (n: number): number => {
    if (n === 0)
        return 0
    else if (n === 1)
        return 1
    else if (1 < n)
        return fib(n - 2) + fib(n - 1)
    else
        throw new Error("Invalid number: " + n)
}

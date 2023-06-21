import { string } from "ts-pattern/dist/patterns"

export class MyProgram {
    static abs = (n: number): number => n < 0 ? -1 * n : n
    private static formatAbs = (x: number): string => `The absolute value of ${x} is ${this.abs(x)}`
    static printAbs = () => console.log(this.formatAbs(-42))

    static factoriol = (n: number): number => {
        let mut_result = 1
        for (let i = 1; i < n + 1; i++) mut_result *= i
        return mut_result
    }
    private static formatFactorial = (n: number): string => `"The factorial of ${n} is ${this.factoriol(n)}.`

    static printAbsAndFactorial = () => {
        console.debug(this.formatAbs(-42))
        console.debug(this.formatFactorial(7))
    }
    static formatResult = (name: string, n: number, f: (arg: number) => number): string => `The ${name} of ${n} is ${f(n)}.`

}

export const fib = (n: number): number => {
    if (n < 0) throw new Error(`Invalid number: ${n}`)
    let current = 0
    let next = 1
    for (let i = 0; i < n; i++) {
        const calculated_next = current + next
        current = next
        next = calculated_next
    }
    return current
}

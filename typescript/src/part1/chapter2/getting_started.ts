export class MyProgram {
    static abs = (n: number): number => n < 0 ? -1 * n : n
    private static formatAbs = (x: number): string => `The absolute value of ${x} is ${this.abs(x)}`
    static printAbs = () => console.log(this.formatAbs(-42))

    static factoriol = (n: number): number => {
        let mut_result = 1
        for (let index = 1; index < n + 1; index++) mut_result *= index
        return mut_result
    }
    private static formatFactorial = (n: number): string => `"The factorial of ${n} is ${this.factoriol(n)}.`

    static printAbsAndFactorial = () => {
        console.debug(this.formatAbs(-42))
        console.debug(this.formatFactorial(7))
    }
    static formatResult = (name: string, n: number, f: (arg: number) => number): string => `The ${name} of ${n} is ${f(n)}.`

    static findFirst = (ss: [string], key: string): number => {
        for (let index = 0; index < ss.length; index++) if (ss[index] == key) return index
        return -1
    }
    static findFirst_ = <A>(as: [A], gt: (arg: A) => boolean): number => {
        for (let index = 0; index < as.length; index++) if (gt(as[index])) return index
        return -1
    }

}

export const fib = (n: number): number => {
    if (n < 0) throw new Error(`Invalid number: ${n}`)
    let current = 0
    let next = 1
    for (let index = 0; index < n; index++) {
        const calculated_next = current + next
        current = next
        next = calculated_next
    }
    return current
}

export const isSorted = <A>(as: Array<A>, gt: (arg1: A, arg2: A) => boolean): boolean => {
    for (let index = 0; index < as.length - 1; index++) {
        const current = as[index]
        const next = as[index + 1]
        if (gt(current, next)) return false
    }
    return true
}

export const curry = <A, B, C>(f: (a: A, b: B) => C): (a: A) => (b: B) => C => {
    return (a: A): (b: B) => C => {
        return (b: B): C => {
            return f(a, b)
        }
    }
}

export const uncurry = <A, B, C>(f: (a: A) => (b: B) => C): (a: A, b: B) => C => {
    return (a: A, b: B): C => {
        return f(a)(b)
    }
}

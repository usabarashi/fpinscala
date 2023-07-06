import { curry, fib, isSorted, uncurry } from 'src/part1/chapter2/getting_started'

describe("Exercise 2.1", () => {
    test("case 0", () => expect(fib(0)).toBe(0))
    test("case 1", () => expect(fib(1)).toBe(1))
    test("case 2", () => expect(fib(2)).toBe(1))
    test("case 3", () => expect(fib(3)).toBe(2))
    test("case 4", () => expect(fib(4)).toBe(3))
    test("case 5", () => expect(fib(5)).toBe(5))
    test("case 6", () => expect(fib(6)).toBe(8))
    test("case 7", () => expect(fib(7)).toBe(13))
    test("case 8", () => expect(fib(8)).toBe(21))
    test("case 9", () => expect(fib(9)).toBe(34))
    test("case -42", () => expect(() => fib(-42)).toThrow(`Invalid number: ${-42}`))
})

describe("Exercise 2.2", () => {
    test("case [1, 2, 3] > true", () => expect(isSorted([1, 2, 3], (arg1, arg2) => arg1 > arg2)).toBe(true))
    test("case [1, 2, 1] > false", () => expect(isSorted([1, 2, 1], (arg1, arg2) => arg1 > arg2)).toBe(false))
    test("case [3, 2, 1] < true", () => expect(isSorted([3, 2, 1], (arg1, arg2) => arg1 < arg2)).toBe(true))
    test("case [1, 2, 3] < false", () => expect(isSorted([1, 2, 3], (arg1, arg2) => arg1 < arg2)).toBe(false))
})

describe("Exercise 2.3", () => {
    const func = (a: number, b: number) => a + b
    test("curry", () => expect(func(42, 42) === curry(func)(42)(42)).toBe(true))
})

describe("Exercise 2.4", () => {
    const func = (a: number) => (b: number) => a + b
    test("uncurry", () => expect(func(42)(42) === uncurry(func)(42, 42)).toBe(true))
})

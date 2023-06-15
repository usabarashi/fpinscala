import { fib } from '../src/exercise21'

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
    test("case -1", () => expect(() => fib(-1)).toThrow("Invalid number: " + -1))
})

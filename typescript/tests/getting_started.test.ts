import { MyProgram } from '../src/getting_started'

describe("Exercise 2.1", () => {
    test("case 0", () => expect(MyProgram.fib(0)).toBe(0))
    test("case 1", () => expect(MyProgram.fib(1)).toBe(1))
    test("case 2", () => expect(MyProgram.fib(2)).toBe(1))
    test("case 3", () => expect(MyProgram.fib(3)).toBe(2))
    test("case 4", () => expect(MyProgram.fib(4)).toBe(3))
    test("case 5", () => expect(MyProgram.fib(5)).toBe(5))
    test("case 6", () => expect(MyProgram.fib(6)).toBe(8))
    test("case 7", () => expect(MyProgram.fib(7)).toBe(13))
    test("case 8", () => expect(MyProgram.fib(8)).toBe(21))
    test("case 9", () => expect(MyProgram.fib(9)).toBe(34))
    test("case -42", () => expect(() => MyProgram.fib(-42)).toThrow(`Invalid number: ${-42}`))
})

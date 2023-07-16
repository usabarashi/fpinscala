import { match, P } from 'ts-pattern'
import { sum, apply, tail, setHead } from 'src/part1/chapter3/functional_data_structure'

describe("Exercise 3.1", () => {
    const result = (): number =>
        match(apply(1, 2, 3, 4, 5))
            .with({ type: 'Cons', head: P.select("x"), tail: { type: 'Cons', head: 2, tail: { type: 'Cons', head: 4 } } }, (params) => params.x)
            .with({ type: 'Nil' }, () => 42)
            .with({ type: 'Cons', head: P.select("x"), tail: { type: 'Cons', head: P.select("y"), tail: { type: 'Cons', head: 3, tail: { type: 'Cons', head: 4 } } } }, (params) => params.x + params.y)
            .with({ type: 'Cons', head: P.select("h"), tail: P.select("t") }, (params) => params.h + sum(params.t))
            .otherwise(() => 101)
    test("List pattern matching", () => expect(result()).toBe(3))
})

describe("Exercise 3.2", () => {
    test("Nil tail", () => expect(() => tail(apply())).toThrow(`Nil`))
    test("Cons tail", () => expect(tail(apply(1, 2, 3))).toEqual(apply(2, 3)))
})

describe("Exercise 3.3", () => {
    test("Nil setHead", () => expect(() => tail(apply())).toThrow(`Nil`))
    test("Cons setHead", () => expect(setHead(apply(1, 2, 3), 42)).toEqual(apply(42, 2, 3)))
})

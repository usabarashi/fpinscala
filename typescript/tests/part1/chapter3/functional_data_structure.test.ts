import { match, P } from 'ts-pattern'
import { List, Nil, Cons, sum, apply, tail, setHead, drop, dropWhile, init, foldRight } from 'src/part1/chapter3/functional_data_structure'

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

describe("Exercise 3.4", () => {
    test("Nil drop", () => expect(drop(apply(), 42)).toEqual(apply()))
    test("Cons drop", () => expect(drop(apply(1, 2, 3, 4, 5), 3)).toEqual(apply(4, 5)))
})

describe("Exercise 3.5", () => {
    test("Nil dropWhile", () => expect(dropWhile(apply<number>(), (n: number) => n < 42)).toEqual(apply()))
    test("Cons dropWhile", () => expect(dropWhile(apply(1, 2, 3, 4, 5), (n: number) => n < 4)).toEqual(apply(4, 5)))
})

describe("Exercise 3.6", () => {
    test("Nil tail", () => expect(() => init(apply())).toThrow(`Nil`))
    test("Cons init", () => expect(init(apply(1, 2, 3, 4, 5))).toEqual(apply(1, 2, 3, 4)))
})

describe("Exercise 3.8", () => {
    test("foldRight", () => expect(foldRight(apply(1, 2, 3, 4, 5), { type: 'Nil' } as List<number>, (head, tail) => ({ type: 'Cons', head: head, tail: tail } as List<number>))).toEqual(apply(1, 2, 3, 4, 5)))
})

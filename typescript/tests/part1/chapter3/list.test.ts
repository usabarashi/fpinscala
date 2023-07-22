import { match, P } from 'ts-pattern'
import {
    List
    , Nil
    , Cons
    , sum
    , apply
    , tail
    , setHead
    , drop
    , dropWhile
    , init
    , foldRight
    , length
    , foldLeft
    , sumLeft
    , productLeft
    , lengthLeft
    , reverse
    , foldLeftFromRight
    , foldRightFromLeft
    , appendRight
    , concat
    , incrementEach
    , doubleToString
    , map
    , filter
    , flatMap
    , filterFromFlatMap
    , addPairwise
    , zipWith
    , take
    , takeWhile
    , forall
    , exists
    , scanLeft
    , scanRight
    , hasSubsequence
} from 'src/part1/chapter3/list'

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

describe("Exercise 3.9", () => {
    test("Nil length", () => expect(length(apply())).toEqual(0))
    test("Cons length", () => expect(length(apply(1, 2, 3))).toEqual(3))
})

describe("Exercise 3.10", () => {
    test("foldLeft", () => expect(foldLeft(apply(1, 2, 3), 0, (a, b) => a + b)).toEqual(6))
})

describe("Exercise 3.11", () => {
    test("sumLeft", () => expect(sumLeft(apply(1, 2, 3))).toEqual(6))
    test("productLeft", () => expect(productLeft(apply(1.0, 2.0, 3.0))).toEqual(6.0))
    test("lengthLeft", () => expect(lengthLeft(apply(1, 2, 3))).toEqual(3))
})

describe("Exercise 3.12", () => {
    test("reverse", () => expect(reverse(apply(1, 2, 3))).toEqual(apply(3, 2, 1)))
})

describe("Exercise 3.13", () => {
    test("foldLeft from foldRight", () => expect(foldLeftFromRight(apply(1, 2, 3), 0, (a, b) => a + b)).toEqual(6))
    test("foldRight from foldLEft", () => expect(foldRightFromLeft(apply(1, 2, 3), 0, (b, a) => b + a)).toEqual(6))
})

describe("Exercise 3.14", () => {
    test("append from foldRight", () => expect(appendRight(apply(1, 2, 3), apply(4, 5, 6))).toEqual(apply(1, 2, 3, 4, 5, 6)))
})

describe("Exercise 3.15", () => {
    test("concat", () => expect(concat(apply(apply(1, 2, 3), apply(4, 5, 6)))).toEqual(apply(1, 2, 3, 4, 5, 6)))
})

describe("Exercise 3.16", () => {
    test("incrementEach", () => expect(incrementEach(apply(1, 2, 3, 4, 5))).toEqual(apply(2, 3, 4, 5, 6)))
})

describe("Exercise 3.17", () => {
    test("doubleToString", () => expect(doubleToString(apply(1.0, 2.0, 3.0, 4.0, 5.0), 1)).toEqual(apply("1.0", "2.0", "3.0", "4.0", "5.0")))
})

describe("Exercise 3.18", () => {
    test("map", () => expect(map(apply(1, 2, 3, 4, 5), (x) => x * 2)).toEqual(apply(2, 4, 6, 8, 10)))
})

describe("Exercise 3.19", () => {
    test("filter", () => expect(filter(apply(1, 2, 3, 4, 5), (x) => x % 2 != 0)).toEqual(apply(1, 3, 5)))
})

describe("Exercise 3.20", () => {
    test("flatMap", () => expect(flatMap(apply(1, 2, 3), (x) => apply(x, x))).toEqual(apply(1, 1, 2, 2, 3, 3)))
})

describe("Exercise 3.21", () => {
    test("filter from flatMap", () => expect(filterFromFlatMap(apply(1, 2, 3), (x) => x % 2 != 0)).toEqual(apply(1, 3)))
})

describe("Exercise 3.22", () => {
    test("addPairwise", () => expect(addPairwise(apply(1, 2, 3), apply(4, 5, 6))).toEqual(apply(5, 7, 9)))
})

describe("Exercise 3.23", () => {
    test("zipWith", () => expect(zipWith(apply(1, 2, 3), apply(4, 5, 6), (a, b) => a + b)).toEqual(apply(5, 7, 9)))
})

describe("LISTS IN THE STANDARD LIBRARY", () => {
    test("take", () => expect(take(apply(1, 2, 3, 4, 5), 3)).toEqual(apply(1, 2, 3)))
    test("takeWhile", () => expect(takeWhile(apply(1, 2, 3, 4, 5), (t) => t <= 3)).toEqual(apply(1, 2, 3)))
    test("forall false", () => expect(forall(apply(1, 2, 3, 4, 5), (t) => t <= 3)).toEqual(false))
    test("forall true", () => expect(forall(apply(1, 2, 3, 4, 5), (t) => t <= 42)).toEqual(true))
    test("exists false", () => expect(exists(apply(1, 2, 3, 4, 5), (t) => t == 42)).toEqual(false))
    test("exists true", () => expect(exists(apply(1, 2, 3, 4, 5), (t) => t == 3)).toEqual(true))
    test("scanLeft", () => expect(scanLeft(apply("a", "b", "c", "d", "e"), "", (x, y) => x + y)).toEqual(apply("", "a", "ab", "abc", "abcd", "abcde")))
    test("scanRight", () => expect(scanRight(apply("a", "b", "c", "d", "e"), "", (x, y) => x + y)).toEqual(apply("abcde", "bcde", "cde", "de", "e", "")))
})

describe("Exercise 3.24", () => {
    test("hasSubsequence false", () => expect(hasSubsequence(apply(1, 2, 3, 4, 5), apply(4, 3, 2))).toEqual(false))
    test("hasSubsequence true", () => expect(hasSubsequence(apply(1, 2, 3, 4, 5), apply(2, 3, 4))).toEqual(true))
})

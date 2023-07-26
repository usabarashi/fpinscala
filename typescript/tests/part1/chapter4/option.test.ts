import {
    Option
    , None
    , Some
    , map
    , flatMap
    , getOrElse
    , orElse
    , filter
} from 'src/part1/chapter4/option'

describe("Exercise 4.1", () => {
    test("map None", () => expect(map({ type: 'None' }, (x) => x)).toStrictEqual({ type: 'None' }))
    test("map Some", () => expect(map({ type: 'Some', value: 42 }, (x) => x)).toStrictEqual({ type: 'Some', value: 42 }))
    test("flatMap None", () => expect(flatMap({ type: 'None' }, (x) => ({ type: 'Some', value: x }))).toStrictEqual({ type: 'None' }))
    test("flatMap Some", () => expect(flatMap({ type: 'Some', value: 42 }, (x) => ({ type: 'Some', value: x }))).toStrictEqual({ type: 'Some', value: 42 }))
    test("getOrElse None", () => expect(getOrElse({ type: 'None' }, () => 0)).toBe(0))
    test("getOrElse Some", () => expect(getOrElse({ type: 'Some', value: 42 }, () => 42)).toBe(42))
    test("orElse None", () => expect(orElse({ type: 'None' }, () => ({ type: 'Some', value: 42 }))).toStrictEqual({ type: 'Some', value: 42 }))
    test("orElse Some", () => expect(orElse({ type: 'Some', value: 42 }, () => ({ type: 'None' }))).toStrictEqual({ type: 'Some', value: 42 }))
    test("filter None", () => expect(filter({ type: 'None' }, (value) => value == 42)).toStrictEqual({ type: 'None' }))
    test("filter Some", () => expect(filter({ type: 'Some', value: 42 }, (value) => value == 42)).toStrictEqual({ type: 'Some', value: 42 }))
})

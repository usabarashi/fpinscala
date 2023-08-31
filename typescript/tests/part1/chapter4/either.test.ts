import {
    Either
    , Left
    , Right
    , map
    , flatMap
    , orElse
    , map2
} from 'src/part1/chapter4/either'

describe("Exercise 4.6", () => {
    test("map Left", () => expect(map({ type: 'Left', value: "error" }, (x) => x)).toStrictEqual({ type: 'Left', value: "error" }))
    test("map Right", () => expect(map({ type: 'Right', value: 42 }, (x) => x)).toStrictEqual({ type: 'Right', value: 42 }))
    test("flatMap Left", () => expect(flatMap({ type: 'Left', value: "error" }, (x) => ({ type: 'Right', value: x }))).toStrictEqual({ type: 'Left', value: "error" }))
    test("flatMap Right", () => expect(flatMap({ type: 'Right', value: 42 }, (x) => ({ type: 'Right', value: x }))).toStrictEqual({ type: 'Right', value: 42 }))
    test("orElse Left", () => expect(orElse({ type: 'Left', value: "error" }, () => ({ type: 'Left', value: "error" }))).toStrictEqual({ type: 'Left', value: "error" }))
    test("orElse Right", () => expect(orElse({ type: 'Right', value: 42 }, () => ({ type: 'Left', value: "error" }))).toStrictEqual({ type: 'Right', value: 42 }))
    test("map2 Left", () => expect(map2({ type: 'Left', value: "error" }, { type: "Left", value: "error" }, (a, b) => a + b)).toStrictEqual({ type: 'Left', value: "error" }))
    test("map2 Right", () => expect(map2({ type: 'Right', value: 42 }, { type: "Right", value: 42 }, (a, b) => a + b)).toStrictEqual({ type: 'Right', value: 84 }))
})

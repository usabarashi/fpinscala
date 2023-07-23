import { match, P } from 'ts-pattern'
import {
    Tree
    , Leaf
    , Branch
    , maximum
    , depth
    , map
} from 'src/part1/chapter3/tree'

describe("Exercise 3.25", () => {
    const tree = {
        type: 'Branch',
        left: {
            type: 'Branch',
            left: { type: 'Leaf', value: 1 },
            right: { type: 'Leaf', value: 2 }
        },
        right: {
            type: 'Branch',
            left: { type: 'Leaf', value: 3 },
            right: { type: 'Leaf', value: 4 }
        },
    } as Tree<number>
    test("Tree maximum", () => expect(maximum(tree)).toBe(4))
})

describe("Exercise 3.26", () => {
    const tree = {
        type: 'Branch',
        left: { type: 'Leaf', value: 1 },
        right: {
            type: 'Branch',
            left: { type: 'Leaf', value: 2 },
            right: {
                type: 'Branch',
                left: { type: 'Leaf', value: 3 },
                right: { type: 'Leaf', value: 3 }
            },
        },
    } as Tree<number>
    test("Tree depth", () => expect(depth(tree)).toBe(3))
})

describe("Exercise 3.27", () => {
    const tree = {
        type: 'Branch',
        left: { type: 'Leaf', value: 1 },
        right: {
            type: 'Branch',
            left: { type: 'Leaf', value: 2 },
            right: {
                type: 'Branch',
                left: { type: 'Leaf', value: 3 },
                right: { type: 'Leaf', value: 3 }
            },
        },
    } as Tree<number>
    const expected = {
        type: 'Branch',
        left: { type: 'Leaf', value: "1" },
        right: {
            type: 'Branch',
            left: { type: 'Leaf', value: "2" },
            right: {
                type: 'Branch',
                left: { type: 'Leaf', value: "3" },
                right: { type: 'Leaf', value: "3" }
            },
        },
    } as Tree<string>
    test("Tree map", () => expect(map(tree, (x) => x.toString())).toStrictEqual(expected))
})

import { match, P } from "ts-pattern";

export type Tree<T> = Leaf<T> | Branch<T>
export interface Leaf<T> {
    type: 'Leaf'
    value: T
}
export interface Branch<T> {
    type: 'Branch'
    left: Tree<T>
    right: Tree<T>
}

const size = <T>(t: Tree<T>): number =>
    match(t)
        .with({ type: 'Leaf' }, () => 1)
        .with({ type: 'Branch' }, (branch) => 1 + size(branch.left) + size(branch.right))
        .exhaustive()

export const maximum = (t: Tree<number>): number =>
    match(t)
        .with({ type: 'Leaf' }, (leaf) => leaf.value)
        .with({ type: 'Branch' }, (branch) => {
            const left = maximum(branch.left)
            const right = maximum(branch.right)
            return left > right ? left : right
        })
        .exhaustive()

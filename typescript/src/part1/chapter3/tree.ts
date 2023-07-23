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

const max = (left: number, right: number): number => left > right ? left : right

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

export const depth = <T>(t: Tree<T>): number =>
    match(t)
        .with({ type: 'Leaf' }, () => 0)
        .with({ type: 'Branch' }, (branch) => 1 + max(depth(branch.left), depth(branch.right)))
        .exhaustive()

export const map = <T, B>(t: Tree<T>, f: (t: T) => B): Tree<B> =>
    match(t)
        .with({ type: 'Leaf' }, (leaf) => ({ ...leaf, value: f(leaf.value) } as Tree<B>))
        .with({ type: 'Branch' }, (branch) => ({ ...branch, left: map(branch.left, f), right: map(branch.right, f) } as Tree<B>))
        .exhaustive()

const fold = <A, B>(t: Tree<A>, f: (a: A) => B, g: (l: B, r: B) => B): B =>
    match(t)
        .with({ type: 'Leaf' }, (leaf) => f(leaf.value))
        .with({ type: 'Branch' }, (branch) => g(fold(branch.left, f, g), fold(branch.right, f, g)))
        .exhaustive()

export const sizeViaFold = <A>(t: Tree<A>): number =>
    fold(t, (_) => 1, (l, r) => 1 + l + r)

export const depthViaFold = <A>(t: Tree<A>): number =>
    fold(t, (_) => 0, (left, right) => 1 + max(left, right))

export const mapViaFold = <A, B>(t: Tree<A>, f: (a: A) => B): Tree<B> =>
    fold(t, (value) => ({ type: 'Leaf', value: f(value) } as Tree<B>), (left, right) => ({ type: 'Branch', left: left, right: right } as Tree<B>))

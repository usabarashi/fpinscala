import { match, P } from "ts-pattern";

export type Either<L, R> = Left<L> | Right<R>
export interface Left<L> {
    type: 'Left'
    value: L
}
export interface Right<R> {
    type: 'Right'
    value: R
}

export const map = <L, A, B>(v: Either<L, A>, f: (v: A) => B): Either<L, B> =>
    match(v)
        .with({ type: 'Left' }, (left) => ({ ...left }))
        .with({ type: 'Right' }, (right) => ({ ...right, value: f(right.value) }))
        .exhaustive()

export const flatMap = <L, A, B>(v: Either<L, A>, f: (v: A) => Either<L, B>): Either<L, B> =>
    match(v)
        .with({ type: 'Left' }, (left) => ({ ...left }))
        .with({ type: 'Right' }, (right) => f(right.value))
        .exhaustive()

export const orElse = <L, LL, R>(v: Either<L, R>, f: () => Either<LL, R>): Either<LL, R> =>
    match(v)
        .with({ type: 'Left' }, () => f())
        .with({ type: 'Right' }, (right) => right)
        .exhaustive()

export const map2 = <L, A, B, C>(a: Either<L, A>, b: Either<L, B>, f: (a: A, b: B) => C): Either<L, C> =>
    flatMap(a, (a) => map(b, (b) => f(a, b)))

export const sequence = <L, R>(xs: Array<Either<L, R>>): Either<L, Array<R>> =>
    traverse(xs, (x) => x)

export const traverse = <L, A, B>(xs: Array<A>, f: (v: A) => Either<L, B>): Either<L, Array<B>> =>
    match(xs)
        .with([], () => ({ type: 'Right', value: [] }) as Either<L, Array<B>>)
        .otherwise((xs) => {
            const [head, ...tail] = xs
            return map2(f(head), traverse(tail, f), (h, t) => [h].concat(t))
        })

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

export const map = <L, R, RR>(v: Either<L, R>, f: (v: R) => RR): Either<L, RR> =>
    match(v)
        .with({ type: 'Left' }, (left) => ({ ...left }))
        .with({ type: 'Right' }, (right) => ({ ...right, value: f(right.value) }))
        .exhaustive()

export const flatMap = <L, R, RR>(v: Either<L, R>, f: (v: R) => Either<L, RR>): Either<L, RR> =>
    match(v)
        .with({ type: 'Left' }, (left) => ({ ...left }))
        .with({ type: 'Right' }, (right) => f(right.value))
        .exhaustive()

export const orElse = <L, LL, R>(v: Either<L, R>, f: () => Either<LL, R>): Either<LL, R> =>
    match(v)
        .with({ type: 'Left' }, () => f())
        .with({ type: 'Right' }, (right) => right)
        .exhaustive()

export const map2 = <L, R1, R2, R3>(a: Either<L, R1>, b: Either<L, R2>, f: (a: R1, b: R2) => R3): Either<L, R3> =>
    flatMap(a, (a) => map(b, (b) => f(a, b)))

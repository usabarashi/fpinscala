import { match, P } from "ts-pattern";

export type Option<T> = None | Some<T>
export interface None {
    type: 'None'
}
export interface Some<T> {
    type: 'Some'
    value: T
}

export const map = <T, B>(t: Option<T>, f: (t: T) => B): Option<B> =>
    match(t)
        .with({ type: 'None' }, (none) => ({ ...none }))
        .with({ type: 'Some' }, (some) => ({ ...some, value: f(some.value) }))
        .exhaustive()

export const flatMap = <T, B>(t: Option<T>, f: (t: T) => Option<B>): Option<B> =>
    getOrElse(map(t, f), () => ({ type: 'None' }))

// def getOrElse[B >: A](default: => B): B =
export const getOrElse = <T>(t: Option<T>, f: () => T): T =>
    match(t)
        .with({ type: 'None' }, () => f())
        .with({ type: 'Some' }, (some) => some.value)
        .exhaustive()

// def orElse[B >: A](ob: => Option[B]): Option[B] =
export const orElse = <T>(t: Option<T>, f: () => Option<T>): Option<T> =>
    getOrElse(map(t, (x) => ({ type: 'Some', value: x })), f)

export const filter = <T>(t: Option<T>, f: (t: T) => boolean): Option<T> =>
    flatMap(t, (x) => f(x) ? { type: 'Some', value: x } : { type: 'None' } as Option<T>)

const mean = (xs: Array<number>): Option<number> =>
    match(xs)
        .with([], () => ({ type: 'None' } as Option<number>))
        .otherwise(() => ({ type: 'Some', value: xs.reduce((accumulator, x) => accumulator + x, 0.0) / xs.length }))

export const variance = (xs: Array<number>): Option<number> =>
    flatMap(mean(xs), (m) => mean(xs.map((x) => Math.pow(x - m, 2))))

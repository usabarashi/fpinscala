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

export const map2 = <A, B, C>(a: Option<A>, b: Option<B>, f: (a: A, b: B) => C): Option<C> =>
    flatMap(a, (a) => map(b, (b) => f(a, b)))

export const sequence = <T>(xs: Array<Option<T>>): Option<Array<T>> =>
    match(xs)
        .with([], () => ({ type: 'Some', value: [] } as Option<Array<T>>))
        .otherwise((xs) => {
            const [head, ...tail] = xs
            return flatMap(head, (h) => map(sequence(tail), (t) => [h].concat(t)))
        })

export const traverse = <A, B>(xs: Array<A>, f: (t: A) => Option<B>): Option<Array<B>> =>
    match(xs)
        .with([], () => ({ type: 'Some', value: [] }) as Option<Array<B>>)
        .otherwise((xs) => {
            const [head, ...tail] = xs
            return map2(f(head), traverse(tail, f), (h, t) => [h].concat(t))
        })

export const sequenceFromTraverse = <T>(xs: Array<Option<T>>): Option<Array<T>> =>
    traverse(xs, (x) => x)

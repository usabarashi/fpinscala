import { match, P } from "ts-pattern";

export type List<T> = Nil<T> | Cons<T>
export interface Nil<T> {
    type: 'Nil'
}
export interface Cons<T> {
    type: 'Cons'
    head: T
    tail: List<T>
}

export const sum = (numbers: List<number>): number =>
    match(numbers)
        .with({ type: 'Nil' }, () => 0)
        .with({ type: 'Cons' }, (cons) => cons.head + sum(cons.tail))
        .exhaustive()

export const apply = <T>(...args: Array<T>): List<T> =>
    args.length === 0
        ? { type: 'Nil' }
        : { type: 'Cons', head: args[0], tail: apply(...args.slice(1)) }

export const tail = <T>(ts: List<T>): List<T> =>
    match(ts)
        .with({ type: 'Nil' }, () => { throw new Error(`Nil`) })
        .with({ type: 'Cons' }, (cons) => cons.tail)
        .exhaustive()

export const setHead = <T>(ts: List<T>, t: T): List<T> =>
    match(ts)
        .with({ type: 'Nil' }, () => { throw new Error('Nil') })
        .with({ type: 'Cons' }, (cons) => ({ ...cons, head: t }))
        .exhaustive();

export const drop = <T>(ts: List<T>, n: number): List<T> =>
    match(ts)
        .with({ type: 'Cons' }, (cons) => 0 < n ? drop(cons.tail, n - 1) : cons)
        .otherwise(() => ts)

export const dropWhile = <T>(ts: List<T>, f: (n: T) => boolean): List<T> =>
    match(ts)
        .with({ type: 'Cons' }, (cons) => f(cons.head) ? dropWhile(cons.tail, f) : cons)
        .otherwise(() => ts)

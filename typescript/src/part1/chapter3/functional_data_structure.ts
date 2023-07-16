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
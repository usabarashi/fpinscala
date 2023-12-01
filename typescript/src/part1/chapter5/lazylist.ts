import { match, P } from "ts-pattern";
import memoize from 'memoizee';

import { Option } from "src/part1/chapter4/option"

export type LazyList<T> = Empty<T> | Cons<T>
export interface Empty<T> {
    type: 'Empty'
}
export interface Cons<T> {
    type: 'Cons'
    head: () => T
    tail: () => LazyList<T>
}

export const empty = <T>(): LazyList<T> => { return { type: 'Empty' } }

export const cons = <T>(head: () => T, tail: () => LazyList<T>): LazyList<T> => {
    const memorized_head = memoize(head)
    const memorized_tail = memoize(tail)
    return {
        type: 'Cons',
        head: () => memorized_head(),
        tail: () => memorized_tail()
    }
}

export const apply = <T>(ts: Array<T>): LazyList<T> => {
    if (ts.length === 0) {
        return empty()
    } else {
        const [head, ...tail] = ts
        return cons(() => head, () => apply(tail))
    }
}

export const headOption = <T>(ts: LazyList<T>): Option<T> =>
    match(ts)
        .with({ type: 'Empty' }, () => ({ type: 'None' } as Option<T>))
        .with({ type: 'Cons' }, (ts) => (ts.head() as Option<T>))
        .exhaustive()

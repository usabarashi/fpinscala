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

export const product = (numbers: List<number>): number =>
    match(numbers)
        .with({ type: 'Nil' }, () => 0)
        .with({ type: 'Cons' }, (cons) => cons.head * product(cons.tail))
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
        .exhaustive()

export const drop = <T>(ts: List<T>, n: number): List<T> =>
    match(ts)
        .with({ type: 'Cons' }, (cons) => 0 < n ? drop(cons.tail, n - 1) : cons)
        .otherwise(() => ts)

export const dropWhile = <T>(ts: List<T>, f: (n: T) => boolean): List<T> =>
    match(ts)
        .with({ type: 'Cons' }, (cons) => f(cons.head) ? dropWhile(cons.tail, f) : cons)
        .otherwise(() => ts)

export const append = <A>(a1: List<A>, a2: List<A>): List<A> =>
    match(a1)
        .with({ type: 'Nil' }, () => a2)
        .with({ type: 'Cons' }, (cons) => ({ ...cons, tail: append(cons.tail, a2) }))
        .exhaustive()

export const init = <T>(ts: List<T>): List<T> =>
    match(ts)
        .with({ type: 'Nil' }, () => { throw new Error('Nil') })
        .with({ type: 'Cons', tail: { type: 'Nil' } }, () => apply<T>())
        .with({ type: 'Cons' }, (cons) => ({ ...cons, tail: init(cons.tail) }))
        .exhaustive()

export const foldRight = <T, B>(ts: List<T>, accumulator: B, f: (head: T, tail: B) => B): B =>
    match(ts)
        .with({ type: 'Nil' }, () => accumulator)
        .with({ type: 'Cons' }, (cons) => f(cons.head, foldRight(cons.tail, accumulator, f)))
        .exhaustive()

export const length = <T>(ts: List<T>): number =>
    foldRight(ts, 0, (_, b) => b + 1)

export const foldLeft = <T, B>(ts: List<T>, accumulator: B, f: (a: B, b: T) => B): B => {
    const isCons = <T>(list: List<T>): list is Cons<T> => list.type === 'Cons'
    let mut_state = accumulator
    let mut_list = ts
    while (isCons(mut_list)) {
        mut_state = f(mut_state, mut_list.head)
        mut_list = mut_list.tail
    }
    return mut_state
}

export const sumLeft = (ts: List<number>): number =>
    foldLeft(ts, 0, (accumulator, t) => accumulator + t)

export const productLeft = (ts: List<number>): number =>
    foldLeft(ts, 1.0, (accumulator, t) => accumulator * t)

export const lengthLeft = <T>(ts: List<T>): number =>
    foldLeft(ts, 0, (accumulator, _) => accumulator + 1)

export const reverse = <T>(ts: List<T>): List<T> =>
    foldLeft(ts, { type: 'Nil' } as List<T>, (accumlator, head) => ({ type: 'Cons', head: head, tail: accumlator }))

export const foldLeftFromRight = <T, B>(ts: List<T>, accumulator: B, f: (a: B, b: T) => B): B =>
    foldRight(ts, accumulator, (b, a) => f(a, b))

export const foldRightFromLeft = <T, B>(ts: List<T>, accumulator: B, f: (b: T, a: B) => B): B =>
    foldLeft(ts, accumulator, (a, b) => f(b, a))

export const appendRight = <A>(a1: List<A>, a2: List<A>): List<A> =>
    foldRight(a1, a2, (head, accumulator) => ({ type: 'Cons', head: head, tail: accumulator }))

export const concat = <A>(l: List<List<A>>): List<A> =>
    foldRight(l, { type: 'Nil' } as List<A>, append)

export const incrementEach = (l: List<number>): List<number> =>
    foldRight(l, { type: 'Nil' } as List<number>, (head, accumulator) => ({ type: 'Cons', head: head + 1, tail: accumulator }))

export const doubleToString = (l: List<number>, decimals: number): List<string> =>
    foldRight(l, { type: 'Nil' } as List<string>, (head, accumulator) => ({ type: 'Cons', head: head.toFixed(decimals), tail: accumulator }))

export const map = <T, B>(l: List<T>, f: (x: T) => B): List<B> =>
    foldRight(l, { type: 'Nil' } as List<B>, (head, accumlator) => ({ type: 'Cons', head: f(head), tail: accumlator }))

export const filter = <T>(l: List<T>, f: (x: T) => boolean): List<T> =>
    foldRight(l, { type: 'Nil' } as List<T>, (head, accumlator) => f(head) ? { type: 'Cons', head: head, tail: accumlator } : accumlator)

export const flatMap = <T, B>(l: List<T>, f: (x: T) => List<B>): List<B> =>
    concat(map(l, f))

export const filterFromFlatMap = <T>(l: List<T>, f: (x: T) => boolean): List<T> =>
    flatMap(l, (x) => f(x) ? apply(x) : { type: 'Nil' })

export const addPairwise = (l1: List<number>, l2: List<number>): List<number> =>
    match([l1, l2])
        .with([{ type: 'Nil' }, P._], () => ({ type: 'Nil' } as List<number>))
        .with([P._, { type: 'Nil' }], () => ({ type: 'Nil' } as List<number>))
        .with([{ type: 'Cons' }, { type: 'Cons' }], (cons) => ({ type: 'Cons', head: cons[0].head + cons[1].head, tail: addPairwise(cons[0].tail, cons[1].tail) } as List<number>))
        .exhaustive()


export const zipWith = <T, B, C>(ts: List<T>, bs: List<B>, f: (t: T, b: B) => C): List<C> =>
    match([ts, bs])
        .with([{ type: 'Nil' }, P._], () => ({ type: 'Nil' } as List<C>))
        .with([P._, { type: 'Nil' }], () => ({ type: 'Nil' } as List<C>))
        .with([{ type: 'Cons' }, { type: 'Cons' }], (cons) => ({ type: 'Cons', head: f(cons[0].head, cons[1].head), tail: zipWith(cons[0].tail, cons[1].tail, f) } as List<C>))
        .exhaustive()

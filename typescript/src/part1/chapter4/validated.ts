import { match, P } from "ts-pattern";
import { Either } from "src/part1/chapter4/either"

export type Validated<E, A> = Invalid<E> | Valid<A>
export interface Invalid<E> {
    type: 'Invalid'
    value: Array<E>
}
export interface Valid<A> {
    type: 'Valid'
    value: A
}

export const toEither = <E, A>(v: Validated<E, A>): Either<Array<E>, A> =>
    match(v)
        .with({ type: 'Invalid' }, (invalid) => ({ type: 'Left', value: invalid.value } as Either<Array<E>, A>))
        .with({ type: 'Valid' }, (valid) => ({ type: 'Right', value: valid.value } as Either<Array<E>, A>))
        .exhaustive()

export const map = <E, A, B>(v: Validated<E, A>, f: (a: A) => B): Validated<E, B> =>
    match(v)
        .with({ type: 'Invalid' }, (invalid) => ({ ...invalid }))
        .with({ type: 'Valid' }, (valid) => ({ ...valid, value: f(valid.value) }))
        .exhaustive()

export const map2 = <E, A, B, C>(a: Validated<E, A>, b: Validated<E, B>, f: (a: A, b: B) => C): Validated<E, C> => {
    if (a.type === 'Valid' && b.type === 'Valid') { return { type: 'Valid', value: f(a.value, b.value) } }
    else if (a.type === 'Invalid' && b.type === 'Valid') { return a }
    else if (a.type === 'Valid' && b.type === 'Invalid') { return b }
    else { return { type: 'Invalid', value: [...(a as Invalid<E>).value, ...(b as Invalid<E>).value] } }
}

export const fromEither = <E, A>(e: Either<Array<E>, A>): Validated<E, A> =>
    match(e)
        .with({ type: 'Left' }, (left) => ({ type: 'Invalid', value: left.value } as Validated<E, A>))
        .with({ type: 'Right' }, (right) => ({ type: 'Valid', value: right.value } as Validated<E, A>))
        .exhaustive()

export const traverse = <E, A, B>(as: Array<A>, f: (a: A) => Validated<E, B>): Validated<E, Array<B>> =>
    as.reduce((acc: Validated<E, Array<B>>, a: A) => map2(f(a), acc, (b: B, bs: Array<B>) => [b, ...bs]), { type: 'Valid', value: [] } as Validated<E, Array<B>>)

export const sequence = <E, A>(vs: Array<Validated<E, A>>): Validated<E, Array<A>> =>
    traverse(vs, (x) => x)

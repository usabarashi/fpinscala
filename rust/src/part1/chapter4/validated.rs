use crate::part1::chapter4::either::Either;
use std::iter::once;

#[derive(Clone)]
enum Validated<E, A> {
    Invalid(Vec<E>),
    Valid(A),
}

impl<E, A> Validated<E, A>
where
    E: Clone,
    A: Clone,
{
    pub fn to_either(&self) -> Either<Vec<E>, A> {
        match self {
            Validated::Invalid(es) => Either::Left(es.clone()),
            Validated::Valid(a) => Either::Right(a.clone()),
        }
    }

    pub fn map<B, F>(&self, f: F) -> Validated<E, B>
    where
        F: Fn(A) -> B + Copy,
    {
        match self {
            Validated::Invalid(es) => Validated::Invalid(es.clone()),
            Validated::Valid(a) => Validated::Valid(f(a.clone())),
        }
    }

    pub fn map2<B, C, F>(&self, b: Validated<E, B>, f: F) -> Validated<E, C>
    where
        F: Fn(A, B) -> C,
    {
        match (self, b) {
            (Validated::Valid(aa), Validated::Valid(bb)) => Validated::Valid(f(aa.clone(), bb)),
            (Validated::Invalid(es), Validated::Valid(_)) => Validated::Invalid(es.clone()),
            (Validated::Valid(_), Validated::Invalid(es)) => Validated::Invalid(es),
            (Validated::Invalid(es1), Validated::Invalid(es2)) => {
                Validated::Invalid(es1.iter().cloned().chain(es2.iter().cloned()).collect())
            }
        }
    }

    pub fn from_either(e: Either<Vec<E>, A>) -> Validated<E, A> {
        match e {
            Either::Right(a) => Validated::Valid(a),
            Either::Left(es) => Validated::Invalid(es),
        }
    }

    fn traverse<B, F>(xs: &Vec<A>, f: F) -> Validated<E, Vec<B>>
    where
        B: Clone,
        F: Fn(A) -> Validated<E, B> + Copy,
    {
        xs.iter().fold(Validated::Valid(Vec::new()), |acc, a| {
            f(a.clone()).map2(acc, |head, tail| once(head).chain(tail).collect())
        })
    }

    fn sequence(xs: &Vec<Validated<E, A>>) -> Validated<E, Vec<A>> {
        Validated::traverse(&xs, |x| x)
    }
}

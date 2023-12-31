use std::iter;

#[derive(Debug, Clone, PartialEq)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R>
where
    L: Clone,
    R: Clone,
{
    fn map<F, RR>(&self, f: F) -> Either<L, RR>
    where
        RR: Clone,
        F: Fn(R) -> RR + Copy,
    {
        match self {
            Either::Left(error) => Either::Left(error.clone()),
            Either::Right(value) => Either::Right(f(value.clone())),
        }
    }

    fn flat_map<F, RR>(&self, f: F) -> Either<L, RR>
    where
        RR: Clone,
        F: Fn(R) -> Either<L, RR> + Copy,
    {
        match self {
            Either::Left(error) => Either::Left(error.clone()),
            Either::Right(value) => f(value.clone()),
        }
    }

    fn or_else<F, LL>(&self, f: F) -> Either<LL, R>
    where
        LL: Clone,
        F: Fn() -> Either<LL, R> + Copy,
    {
        match self {
            Either::Left(_) => f(),
            Either::Right(value) => Either::Right(value.clone()),
        }
    }

    fn map2<F, R2, R3>(&self, other: &Either<L, R2>, f: F) -> Either<L, R3>
    where
        R2: Clone,
        R3: Clone,
        F: Fn(R, R2) -> R3 + Copy,
    {
        self.flat_map(|r1| other.map(|r2| f(r1.clone(), r2)))
    }
}

fn sequence<E, A>(xs: &[Either<E, A>]) -> Either<E, Vec<A>>
where
    E: Clone,
    A: Clone,
{
    traverse(xs, |x| x)
}

fn traverse<E, A, B, F>(xs: &[A], f: F) -> Either<E, Vec<B>>
where
    E: Clone,
    A: Clone,
    B: Clone,
    F: Fn(A) -> Either<E, B> + Copy,
{
    match xs {
        [] => Either::Right(vec![]),
        [x, xs @ ..] => {
            let other = traverse(xs, f);
            f(x.clone()).map2(&other, |head, tail| iter::once(head).chain(tail).collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise46() {
        let left = Either::<&str, usize>::Left("error");
        let right = Either::<&str, usize>::Right(42);

        let left_map = left.map(|value| value);
        let right_map = right.map(|value| value);
        assert_eq!(left_map, left);
        assert_eq!(right_map, right);

        let left_flat_map = left.flat_map(|value| Either::Right(value));
        let right_flat_map = right.flat_map(|value| Either::Right(value));
        assert_eq!(left_flat_map, left);
        assert_eq!(right_flat_map, right);

        let left_or_else = left.or_else(|| Either::Left("error"));
        let right_or_else = right.or_else(|| Either::Left("error"));
        assert_eq!(left_or_else, left);
        assert_eq!(right_or_else, right);

        let left_map2 = left.map2(&left, |x, y| x + y);
        let right_map2 = right.map2(&right, |x, y| x + y);
        assert_eq!(left_map2, left);
        assert_eq!(right_map2, Either::Right(84));
    }

    #[test]
    fn test_exercise47() {
        assert_eq!(
            sequence(&[Either::Right(42), Either::Left("error")]),
            Either::Left("error")
        );
        assert_eq!(
            sequence(&[Either::<&str, u32>::Right(42), Either::Right(42)]),
            Either::Right(vec![42, 42])
        );
        assert_eq!(
            traverse(&[Either::Right(42), Either::Left("error")], |x| x),
            Either::Left("error")
        );
        assert_eq!(
            traverse(&[Either::<&str, u32>::Right(42), Either::Right(42)], |x| x),
            Either::Right(vec![42, 42])
        );
    }
}

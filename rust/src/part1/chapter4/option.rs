use std::iter;

#[derive(Debug, Clone, PartialEq)]
enum MyOption<A> {
    MyNone,
    MySome(A),
}

impl<A> MyOption<A>
where
    A: Clone,
{
    fn map<B, F>(&self, f: F) -> MyOption<B>
    where
        F: Fn(A) -> B + Copy,
    {
        match self {
            MyOption::MyNone => MyOption::MyNone,
            MyOption::MySome(value) => MyOption::MySome(f(value.clone())),
        }
    }

    fn flat_map<B, F>(&self, f: F) -> MyOption<B>
    where
        B: Clone,
        F: Fn(A) -> MyOption<B> + Copy,
    {
        self.map(f).get_or_else(|| MyOption::MyNone)
    }

    /// def getOrElse[B >: A](default: => B): B
    fn get_or_else<F>(&self, default: F) -> A
    where
        F: Fn() -> A + Copy,
    {
        match self {
            MyOption::MyNone => default(),
            MyOption::MySome(value) => value.clone(),
        }
    }

    /// def orElse[B >: A](ob: => Option[B]): Option[B]
    fn or_else<F>(&self, ob: F) -> MyOption<A>
    where
        F: Fn() -> MyOption<A> + Copy,
    {
        self.map(|value| MyOption::MySome(value)).get_or_else(ob)
    }

    fn filter<F>(&self, f: F) -> MyOption<A>
    where
        F: Fn(A) -> bool,
    {
        self.flat_map(|value| {
            if f(value.clone()) {
                MyOption::MySome(value)
            } else {
                MyOption::MyNone
            }
        })
    }

    fn map2<B, C, F>(&self, other: MyOption<B>, f: F) -> MyOption<C>
    where
        B: Clone,
        C: Clone,
        F: Fn(A, B) -> C + Copy,
    {
        self.flat_map(|a| other.map(|b| f(a.clone(), b)))
    }
}

fn mean(xs: &[f64]) -> MyOption<f64> {
    if xs.is_empty() {
        MyOption::MyNone
    } else {
        MyOption::MySome(xs.iter().sum::<f64>() / xs.len() as f64)
    }
}

fn variance(xs: &[f64]) -> MyOption<f64> {
    mean(xs).flat_map(|m| mean(&xs.iter().map(|x| (x - m).powf(2.0)).collect::<Vec<_>>()))
}

fn sequence<A>(xs: &[MyOption<A>]) -> MyOption<Vec<A>>
where
    A: Clone,
{
    match xs {
        [] => MyOption::MySome(Vec::<A>::new()),
        [x, xs @ ..] => (*x).flat_map(|head| {
            sequence(xs).map(|tail| iter::once(head.clone()).chain(tail).collect())
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise41() {
        //assert_eq!(MyOption::MyNone.map(|x| x + 1), MyOption::MyNone);
        assert_eq!(MyOption::MySome(42).map(|x| x + 1), MyOption::MySome(43));
        //assert_eq!(MyOption::MyNone.flat_map(|x| x + 1), MyOption::MyNone);
        assert_eq!(
            MyOption::MySome(42).flat_map(|x| MyOption::MySome(x + 1)),
            MyOption::MySome(43)
        );
        assert_eq!(MyOption::MyNone.get_or_else(|| 0), 0);
        assert_eq!(MyOption::MySome(42).get_or_else(|| 0), 42);
        assert_eq!(
            MyOption::MyNone.or_else(|| MyOption::MySome(0)),
            MyOption::MySome(0)
        );
        assert_eq!(
            MyOption::MySome(42).or_else(|| MyOption::MySome(0)),
            MyOption::MySome(42)
        );
        //assert_eq!(MyOption::MyNone.filter(|x| <= 42), MyOption::MyNone);
        assert_eq!(
            MyOption::MySome(42).filter(|x| x <= 42),
            MyOption::MySome(42)
        );
    }

    #[test]
    fn test_exercise42() {
        assert_eq!(variance(&[1.0, 2.0, 3.0, 4.0, 5.0]), MyOption::MySome(2.0));
    }

    #[test]
    fn test_exercise43() {
        assert_eq!(
            MyOption::MySome(42).map2::<i32, i32, _>(MyOption::MyNone, |a, b| a + b),
            MyOption::MyNone,
        );
        assert_eq!(
            MyOption::MySome(42).map2::<i32, i32, _>(MyOption::MySome(42), |a, b| a + b),
            MyOption::MySome(84)
        );
    }

    #[test]
    fn test_exercise44() {
        assert_eq!(
            sequence(&[MyOption::MySome(42), MyOption::MyNone]),
            MyOption::MyNone
        );
        assert_eq!(
            sequence(&[MyOption::MySome(42), MyOption::MySome(42)]),
            MyOption::MySome(vec![42, 42])
        );
    }
}

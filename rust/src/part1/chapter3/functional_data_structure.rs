enum List<A> {
    Nil,
    Cons { head: A, tail: Box<List<A>> },
}

impl<A> List<A>
where
    A: Clone,
{
    fn sum(ints: &List<i32>) -> i32 {
        match ints {
            List::Nil => 0,
            List::Cons { head, tail } => head + Self::sum(tail),
        }
    }

    fn product(doubles: &List<f64>) -> f64 {
        match doubles {
            List::Nil => 1.0,
            List::Cons { head, tail: _ } if *head == 0.0 => 0.0,
            List::Cons { head, tail } => head * Self::product(tail),
        }
    }

    fn apply(as_: &[A]) -> List<A> {
        if as_.is_empty() {
            List::Nil
        } else {
            List::Cons {
                head: as_[0].clone(),
                tail: Box::new(Self::apply(&as_[1..])),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exercise31() {
        fn result() -> usize {
            match [1, 2, 3, 4, 5] {
                [x, 2, 4, _, _] => x,
                // [] => 42,
                [x, y, 3, 4, _] => x + y,
                [h, t @ ..] => h + t.iter().sum::<usize>(),
                // _ => 101,
            }
        }
        assert_eq!(result(), 3)
    }
}

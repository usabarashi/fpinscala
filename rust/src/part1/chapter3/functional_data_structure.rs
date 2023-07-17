use std::fmt;
use std::rc::Rc;

enum List<A> {
    Nil,
    Cons { head: A, tail: Rc<List<A>> },
}

impl<A> List<A>
where
    A: Clone,
{
    fn new(as_: &[A]) -> List<A> {
        if as_.is_empty() {
            List::Nil
        } else {
            List::Cons {
                head: as_[0].clone(),
                tail: Rc::new(Self::new(&as_[1..])),
            }
        }
    }

    fn tail(&self) -> &List<A> {
        match self {
            List::Nil => panic!("Nil"),
            List::Cons { head, tail } => tail,
        }
    }

    fn set_head(&self, a: A) -> List<A> {
        match self {
            List::Nil => panic!("Nil"),
            List::Cons { head, tail } => List::Cons {
                head: a,
                tail: Rc::clone(&tail),
            },
        }
    }

    fn drop_(&self, n: u32) -> &List<A> {
        match self {
            List::Nil => self,
            List::Cons { .. } if n <= 0 => self,
            List::Cons { head, tail } => tail.drop_(n - 1),
        }
    }

    fn drop_while(&self, f: &dyn Fn(&A) -> bool) -> &List<A> {
        match self {
            List::Cons { head, tail } if f(head) => tail.drop_while(f),
            _ => self,
        }
    }

    fn init(&self) -> List<A> {
        match self {
            List::Nil => panic!("Nil"),
            List::Cons {
                head: source_head,
                tail: source_tail,
            } => match Rc::as_ref(source_tail) {
                List::Nil => List::Nil {},
                List::Cons { .. } => List::Cons {
                    head: source_head.clone(),
                    tail: Rc::new(source_tail.init()),
                },
            },
        }
    }

    fn fold_right<B>(&self, accumulator: &B, f: &dyn Fn(&A, &B) -> B) -> B
    where
        B: Clone,
    {
        match self {
            List::Nil => accumulator.clone(),
            List::Cons { head, tail } => f(head, &tail.fold_right(accumulator, f)),
        }
    }

    fn length(&self) -> usize {
        self.fold_right(&0, &|&_, &b| b + 1)
    }

    fn fold_left<B>(&self, accumulator: &B, f: &dyn Fn(&B, &A) -> B) -> B
    where
        B: Clone,
    {
        let mut list = self;
        let mut state = accumulator.clone();
        while let List::Cons { head, tail } = list {
            state = f(&state, head);
            list = tail;
        }
        state
    }
}

impl List<i32> {
    fn sum(&self) -> i32 {
        match self {
            List::Nil => 0,
            List::Cons { head, tail } => head + tail.sum(),
        }
    }
}

impl List<f64> {
    fn product(&self) -> f64 {
        match self {
            List::Nil => 1.0,
            List::Cons { head, tail: _ } if *head == 0.0 => 0.0,
            List::Cons { head, tail } => head * tail.product(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            List::Nil => write!(f, "Nil"),
            List::Cons { head, tail } => write!(f, "Cons {{ head: {:?}, tail: {:?} }}", head, tail),
        }
    }
}

impl<T: PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (List::Nil, List::Nil) => true,
            (List::Nil, List::Cons { .. }) | (List::Cons { .. }, List::Nil) => false,
            (
                List::Cons {
                    head: self_head,
                    tail: self_tail,
                },
                List::Cons {
                    head: other_head,
                    tail: other_tail,
                },
            ) => self_head == other_head && self_tail == other_tail,
        }
    }
}

impl<A: Clone> Clone for List<A> {
    fn clone(&self) -> Self {
        match self {
            List::Nil => List::Nil,
            List::Cons { head, tail } => List::Cons {
                head: head.clone(),
                tail: Rc::clone(tail),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise31() {
        fn result() -> usize {
            match [1, 2, 3, 4, 5] {
                [x, 2, 4, _, _] => x,
                // [] => 42,
                [x, y, 3, 4, _] => x + y,
                [h, t @ ..] => h + t.iter().sum::<usize>(),
                // _ => 101,
            }
        }
        assert_eq!(result(), 3);
    }

    #[test]
    fn test_exercise32() {
        assert_eq!(
            List::<i32>::new(&[1, 2, 3]).tail(),
            &List::<i32>::new(&[2, 3])
        );
    }

    #[test]
    fn test_exercise33() {
        assert_eq!(
            List::<i32>::new(&[1, 2, 3]).set_head(42),
            List::<i32>::new(&[42, 2, 3])
        );
    }

    #[test]
    fn test_exercise34() {
        assert_eq!(List::<i32>::new(&[]).drop_(42), &List::<i32>::new(&[]));
        assert_eq!(List::new(&[1, 2, 3, 4, 5]).drop_(3), &List::new(&[4, 5]));
    }

    #[test]
    fn test_exercise35() {
        assert_eq!(
            List::<i32>::new(&[]).drop_while(&|&n| n < 42),
            &List::<i32>::new(&[])
        );
        assert_eq!(
            List::new(&[1, 2, 3, 4, 5]).drop_while(&|&n| n < 4),
            &List::new(&[4, 5])
        );
    }

    #[test]
    fn test_exercise36() {
        assert_eq!(List::new(&[1, 2, 3, 4, 5]).init(), List::new(&[1, 2, 3, 4]));
    }

    #[test]
    #[should_panic]
    fn test_exercise36_panic() {
        List::new(&[1]).init().init();
    }

    #[test]
    fn test_exercise38() {
        assert_eq!(
            List::new(&[1, 2, 3]).fold_right(&Rc::new(List::Nil), &|head, tail| Rc::new(
                List::Cons {
                    head: head.clone(),
                    tail: Rc::clone(tail)
                }
            )),
            Rc::new(List::new(&[1, 2, 3]))
        );
    }

    #[test]
    fn test_exercise39() {
        assert_eq!(List::<i32>::Nil.length(), 0);
        assert_eq!(List::new(&[1, 2, 3]).length(), 3);
    }

    #[test]
    fn test_exercise310() {
        assert_eq!(List::new(&[1, 2, 3]).fold_left(&0, &|a, b| a + b), 6);
    }
}

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

    fn drop_while(&self, f: fn(&A) -> bool) -> &List<A> {
        match self {
            List::Cons { head, tail } if f(head) => tail.drop_while(f),
            _ => self,
        }
    }

    fn append(&self, a2: List<A>) -> List<A> {
        match self {
            List::Nil => a2,
            List::Cons { head, tail } => List::Cons {
                head: head.clone(),
                tail: Rc::new(tail.append(a2)),
            },
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

    fn fold_right<B, F>(&self, accumulator: B, f: F) -> B
    where
        F: Fn(&A, B) -> B + Copy, // 環境をキャプチャしない純粋関数はCopy可能
    {
        match self {
            List::Nil => accumulator,
            List::Cons { head, tail } => f(head, tail.fold_right(accumulator, f)),
        }
    }

    fn length(&self) -> usize {
        self.fold_right(0, |&_, b| b + 1)
    }

    fn fold_left<B, F>(&self, accumulator: B, f: F) -> B
    where
        F: Fn(B, &A) -> B,
    {
        let mut list = self;
        let mut state = accumulator;
        while let List::Cons { head, tail } = list {
            state = f(state, head);
            list = tail;
        }
        state
    }

    fn length_left(&self) -> usize {
        self.fold_left(0, |a, _| a + 1)
    }

    fn reverse(&self) -> List<A> {
        self.fold_left(List::<A>::Nil, |accumulator, head| List::Cons {
            head: head.clone(),
            tail: Rc::new(accumulator),
        })
    }

    fn fold_left_from_right<B, F>(&self, accumulator: B, f: F) -> B
    where
        F: Fn(B, &A) -> B,
    {
        self.fold_right(accumulator, |b, a| f(a, b))
    }

    fn fold_right_from_left<B, F>(&self, accumulator: B, f: F) -> B
    where
        F: Fn(&A, B) -> B,
    {
        self.fold_left(accumulator, |a, b| f(b, a))
    }

    fn append_right(&self, a2: List<A>) -> List<A> {
        self.fold_right(a2, |head, accumulator| List::Cons {
            head: head.clone(),
            tail: Rc::new(accumulator),
        })
    }
}

impl List<i32> {
    fn sum(&self) -> i32 {
        match self {
            List::Nil => 0,
            List::Cons { head, tail } => head + tail.sum(),
        }
    }

    fn sum_left(&self) -> i32 {
        self.fold_left(0, |a, t| a + t)
    }

    fn increment_each(&self) -> List<i32> {
        self.fold_right(List::<i32>::Nil, |head, accumulator| List::Cons {
            head: head + 1,
            tail: Rc::new(accumulator),
        })
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

    fn product_left(&self) -> f64 {
        self.fold_left(1.0, |a, t| a * t)
    }

    fn double_to_string(&self, decimals: usize) -> List<String> {
        self.fold_right(List::<String>::Nil, |head, accumulator| List::Cons {
            head: format!("{:.*}", decimals, head),
            tail: Rc::new(accumulator),
        })
    }
}

impl<A> List<List<A>>
where
    A: Clone,
{
    fn concat(&self) -> List<A> {
        self.fold_right(List::<A>::Nil, |head, accumulator| head.append(accumulator))
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
            List::<i32>::new(&[]).drop_while(|&n| n < 42),
            &List::<i32>::new(&[])
        );
        assert_eq!(
            List::new(&[1, 2, 3, 4, 5]).drop_while(|&n| n < 4),
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
            List::new(&[1, 2, 3]).fold_right(List::Nil, |head, tail| List::Cons {
                head: head.clone(),
                tail: Rc::new(tail)
            }),
            List::new(&[1, 2, 3])
        );
    }

    #[test]
    fn test_exercise39() {
        assert_eq!(List::<i32>::Nil.length(), 0);
        assert_eq!(List::new(&[1, 2, 3]).length(), 3);
    }

    #[test]
    fn test_exercise310() {
        assert_eq!(List::new(&[1, 2, 3]).fold_left(0, |a, b| a + b), 6);
    }

    #[test]
    fn test_exercise311() {
        assert_eq!(List::new(&[1, 2, 3]).sum_left(), 6);
        assert_eq!(List::new(&[1.0, 2.0, 3.0]).product_left(), 6.0);
        assert_eq!(List::new(&[1, 2, 3]).length(), 3);
    }

    #[test]
    fn test_exercise312() {
        assert_eq!(List::new(&[1, 2, 3]).reverse(), List::new(&[3, 2, 1]));
    }

    #[test]
    fn test_exercise313() {
        assert_eq!(
            List::new(&[1, 2, 3]).fold_left_from_right(0, |a, b| a + b),
            6
        );
        assert_eq!(
            List::new(&[1, 2, 3]).fold_right_from_left(0, |b, a| b + a),
            6
        );
    }

    #[test]
    fn test_exercise314() {
        assert_eq!(
            List::new(&[1, 2, 3]).append_right(List::new(&[4, 5, 6])),
            List::new(&[1, 2, 3, 4, 5, 6])
        );
    }

    #[test]
    fn test_exercise315() {
        let l1 = List::new(&[1, 2, 3]);
        let l2 = List::new(&[4, 5, 6]);
        assert_eq!(
            List::new(&[l1, l2]).concat(),
            List::new(&[1, 2, 3, 4, 5, 6])
        );
    }

    #[test]
    fn test_exercise316() {
        assert_eq!(
            List::new(&[1, 2, 3, 4, 5]).increment_each(),
            List::new(&[2, 3, 4, 5, 6])
        );
    }

    #[test]
    fn test_exercise317() {
        assert_eq!(
            List::<f64>::new(&[1.0, 2.0, 3.0, 4.0, 5.0]).double_to_string(1),
            List::new(&[
                "1.0".to_string(),
                "2.0".to_string(),
                "3.0".to_string(),
                "4.0".to_string(),
                "5.0".to_string()
            ])
        );
    }
}

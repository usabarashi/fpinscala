use std::fmt;
use std::rc::Rc;

enum List<A> {
    Nil,
    Cons(A, Rc<List<A>>),
}

impl<A> List<A>
where
    A: Clone + PartialEq,
{
    fn new(as_: &[A]) -> List<A> {
        if as_.is_empty() {
            List::Nil
        } else {
            List::Cons(as_[0].clone(), Rc::new(Self::new(&as_[1..])))
        }
    }

    fn tail(&self) -> &List<A> {
        match self {
            List::Nil => panic!("Nil"),
            List::Cons(head, tail) => tail,
        }
    }

    fn set_head(&self, a: A) -> List<A> {
        match self {
            List::Nil => panic!("Nil"),
            List::Cons(head, tail) => List::Cons(a, Rc::clone(&tail)),
        }
    }

    fn drop_(&self, n: u32) -> &List<A> {
        match self {
            List::Nil => self,
            List::Cons(_, _) if n <= 0 => self,
            List::Cons(_, tail) => tail.drop_(n - 1),
        }
    }

    fn drop_while(&self, f: fn(&A) -> bool) -> &List<A> {
        match self {
            List::Cons(head, tail) if f(head) => tail.drop_while(f),
            _ => self,
        }
    }

    fn append(&self, a2: List<A>) -> List<A> {
        match self {
            List::Nil => a2,
            List::Cons(head, tail) => List::Cons(head.clone(), Rc::new(tail.append(a2))),
        }
    }

    fn init(&self) -> List<A> {
        match self {
            List::Nil => panic!("Nil"),
            List::Cons(source_head, source_tail) => match Rc::as_ref(source_tail) {
                List::Nil => List::Nil {},
                List::Cons(_, _) => List::Cons(source_head.clone(), Rc::new(source_tail.init())),
            },
        }
    }

    fn fold_right<B, F>(&self, accumulator: B, f: F) -> B
    where
        F: Fn(&A, B) -> B + Copy, // 環境をキャプチャしない純粋関数はCopy可能
    {
        match self {
            List::Nil => accumulator,
            List::Cons(head, tail) => f(head, tail.fold_right(accumulator, f)),
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
        while let List::Cons(head, tail) = list {
            state = f(state, head);
            list = tail;
        }
        state
    }

    fn length_left(&self) -> usize {
        self.fold_left(0, |a, _| a + 1)
    }

    fn reverse(&self) -> List<A> {
        self.fold_left(List::<A>::Nil, |accumulator, head| {
            List::Cons(head.clone(), Rc::new(accumulator))
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
        self.fold_right(a2, |head, accumulator| {
            List::Cons(head.clone(), Rc::new(accumulator))
        })
    }

    fn map<B, F>(&self, f: F) -> List<B>
    where
        F: Fn(A) -> B,
    {
        self.fold_right(List::<B>::Nil, |head, accumulator| {
            List::Cons(f(head.clone()), Rc::new(accumulator))
        })
    }

    fn filter<F>(&self, f: F) -> List<A>
    where
        F: Fn(A) -> bool,
    {
        self.fold_right(List::<A>::Nil, |head, accumulator| {
            // FIXME
            if f(head.clone()) {
                List::Cons(head.clone(), Rc::new(accumulator))
            } else {
                accumulator
            }
        })
    }

    fn flat_map<B, F>(&self, f: F) -> List<B>
    where
        B: Clone + PartialEq,
        F: Fn(A) -> List<B> + Copy,
    {
        self.map(f).concat()
    }

    fn filter_from_flat_map<F>(&self, f: F) -> List<A>
    where
        F: Fn(A) -> bool,
    {
        self.flat_map(|x| {
            // FIXME
            if f(x.clone()) {
                List::new(&[x.clone()])
            } else {
                List::Nil
            }
        })
    }

    fn zip_with<B, C, F>(&self, other: List<B>, f: F) -> List<C>
    where
        B: Clone,
        F: Fn(A, B) -> C,
    {
        match (self, other) {
            (List::Nil, _) | (_, List::Nil) => List::Nil,
            (List::Cons(shead, stail), List::Cons(ohead, otail)) => List::Cons(
                f(shead.clone(), ohead),
                Rc::new(stail.zip_with((*otail).clone(), f)),
            ),
        }
    }

    fn take(&self, n: usize) -> List<A> {
        match self {
            List::Nil => List::Nil,
            List::Cons(_, _) if n <= 0 => List::Nil,
            List::Cons(head, tail) => List::Cons(head.clone(), Rc::new(tail.take(n - 1))),
        }
    }

    fn take_while<F>(&self, f: F) -> List<A>
    where
        F: Fn(&A) -> bool,
    {
        match self {
            List::Nil => List::Nil,
            List::Cons(head, tail) if !f(head) => List::Nil,
            List::Cons(head, tail) => List::Cons(head.clone(), Rc::new(tail.take_while(f))),
        }
    }

    fn forall<F>(&self, f: F) -> bool
    where
        F: Fn(A) -> bool,
    {
        self.fold_left(true, |accumulator, head| accumulator && f(head.clone()))
    }

    fn exists<F>(&self, f: F) -> bool
    where
        F: Fn(A) -> bool,
    {
        self.fold_left(false, |accumulator, head| accumulator || f(head.clone()))
    }

    fn scan_left<F, B>(&self, accumulator: B, f: F) -> List<B>
    where
        B: Clone + PartialEq,
        F: Fn(B, A) -> B + Copy,
    {
        match self {
            List::Nil => List::new(&[accumulator.clone()]),
            List::Cons(head, tail) => List::Cons(
                accumulator.clone(),
                Rc::new(tail.scan_left(f(accumulator, head.clone()), f)),
            ),
        }
    }

    fn head(&self) -> A {
        match self {
            List::Nil => panic!("Nil"),
            List::Cons(head, tail) => head.clone(),
        }
    }

    fn scan_right<F, B>(&self, accumulator: B, f: F) -> List<B>
    where
        B: Clone + PartialEq,
        F: Fn(A, B) -> B + Copy,
    {
        match self {
            List::Nil => List::new(&[accumulator.clone()]),
            List::Cons(head, tail) => {
                let new_tail = tail.scan_right(accumulator, f);
                let new_head = f(head.clone(), new_tail.head());
                List::Cons(new_head, Rc::new(new_tail))
            }
        }
    }

    fn start_with(&self, prefix: &List<A>) -> bool {
        match (self, prefix) {
            (_, List::Nil) => true,
            (List::Cons(shead, stail), List::Cons(phead, ptail)) if shead == phead => {
                stail.start_with(ptail)
            }
            _ => false,
        }
    }

    fn has_subsequence(&self, sub: &List<A>) -> bool {
        match self {
            List::Nil => sub.clone() == List::Nil,
            _ if self.start_with(sub) => true,
            List::Cons(head, tail) => tail.has_subsequence(sub),
        }
    }
}

impl List<i32> {
    fn sum(&self) -> i32 {
        match self {
            List::Nil => 0,
            List::Cons(head, tail) => head + tail.sum(),
        }
    }

    fn sum_left(&self) -> i32 {
        self.fold_left(0, |a, t| a + t)
    }

    fn increment_each(&self) -> List<i32> {
        self.fold_right(List::<i32>::Nil, |head, accumulator| {
            List::Cons(head + 1, Rc::new(accumulator))
        })
    }

    fn add_pairwise(&self, b: List<i32>) -> List<i32> {
        match (self, b) {
            (List::Nil, _) | (_, List::Nil) => List::Nil,
            (List::Cons(h1, t1), List::Cons(h2, t2)) => List::Cons(
                h1 + h2,
                // FIXME
                Rc::new(t1.add_pairwise((*t2).clone())),
            ),
        }
    }
}

impl List<f64> {
    fn product(&self) -> f64 {
        match self {
            List::Nil => 1.0,
            List::Cons(head, _) if *head == 0.0 => 0.0,
            List::Cons(head, tail) => head * tail.product(),
        }
    }

    fn product_left(&self) -> f64 {
        self.fold_left(1.0, |a, t| a * t)
    }

    fn double_to_string(&self, decimals: usize) -> List<String> {
        self.fold_right(List::<String>::Nil, |head, accumulator| {
            List::Cons(format!("{:.*}", decimals, head), Rc::new(accumulator))
        })
    }
}

impl<A> List<List<A>>
where
    A: Clone + PartialEq,
{
    fn concat(&self) -> List<A> {
        self.fold_right(List::<A>::Nil, |head, accumulator| head.append(accumulator))
    }
}

impl<T: fmt::Debug> fmt::Debug for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            List::Nil => write!(f, "Nil"),
            List::Cons(head, tail) => write!(f, "Cons {{ head: {:?}, tail: {:?} }}", head, tail),
        }
    }
}

impl<T: PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (List::Nil, List::Nil) => true,
            (List::Nil, List::Cons { .. }) | (List::Cons { .. }, List::Nil) => false,
            (List::Cons(self_head, self_tail), List::Cons(other_head, other_tail)) => {
                self_head == other_head && self_tail == other_tail
            }
        }
    }
}

impl<A: Clone> Clone for List<A> {
    fn clone(&self) -> Self {
        match self {
            List::Nil => List::Nil,
            List::Cons(head, tail) => List::Cons(head.clone(), Rc::clone(tail)),
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
            List::new(&[1, 2, 3]).fold_right(List::Nil, |head, tail| List::Cons(
                head.clone(),
                Rc::new(tail)
            )),
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

    #[test]
    fn test_exercise318() {
        assert_eq!(
            List::new(&[1, 2, 3, 4, 5]).map(|i| i * 2),
            List::new(&[2, 4, 6, 8, 10])
        );
    }

    #[test]
    fn test_exercise319() {
        assert_eq!(
            List::new(&[1, 2, 3, 4, 5]).filter(|i| i % 2 != 0),
            List::new(&[1, 3, 5])
        );
    }

    #[test]
    fn test_exercise320() {
        assert_eq!(
            List::new(&[1, 2, 3]).flat_map(|i| List::new(&[i, i])),
            List::new(&[1, 1, 2, 2, 3, 3])
        );
    }

    #[test]
    fn test_exercise321() {
        assert_eq!(
            List::new(&[1, 2, 3]).filter_from_flat_map(|i| i % 2 != 0),
            List::new(&[1, 3])
        );
    }

    #[test]
    fn test_exercise322() {
        assert_eq!(
            List::new(&[1, 2, 3]).add_pairwise(List::new(&[4, 5, 6])),
            List::new(&[5, 7, 9])
        );
    }

    #[test]
    fn test_exercise323() {
        assert_eq!(
            List::new(&[1, 2, 3]).zip_with(List::new(&[4, 5, 6]), |a, b| a + b),
            List::new(&[5, 7, 9])
        );
    }

    #[test]
    fn test_lists_in_the_standard_library() {
        assert_eq!(List::new(&[1, 2, 3, 4, 5]).take(3), List::new(&[1, 2, 3]));
        assert_eq!(
            List::new(&[1, 2, 3, 4, 5]).take_while(|&x| x <= 3),
            List::new(&[1, 2, 3])
        );
        assert_eq!(
            List::new(&[1, 2, 3, 4, 5]).take_while(|&x| x <= 42),
            List::new(&[1, 2, 3, 4, 5])
        );
        assert_eq!(List::new(&[1, 2, 3, 4, 5]).forall(|x| x <= 3), false);
        assert_eq!(List::new(&[1, 2, 3, 4, 5]).forall(|x| x <= 42), true);
        assert_eq!(List::new(&[1, 2, 3, 4, 5]).exists(|x| x == 42), false);
        assert_eq!(List::new(&[1, 2, 3, 4, 5]).exists(|x| x == 3), true);
        assert_eq!(
            List::new(&["a", "b", "c", "d", "e"])
                .scan_left(String::from(""), |x, y| format!("{}{}", x, y)),
            List::new(&[
                String::from(""),
                String::from("a"),
                String::from("ab"),
                String::from("abc"),
                String::from("abcd"),
                String::from("abcde")
            ])
        );
        assert_eq!(
            List::new(&["a", "b", "c", "d", "e"])
                .scan_right(String::from(""), |x, y| format!("{}{}", x, y)),
            List::new(&[
                String::from("abcde"),
                String::from("bcde"),
                String::from("cde"),
                String::from("de"),
                String::from("e"),
                String::from(""),
            ])
        );
    }

    #[test]
    fn test_exercise324() {
        assert_eq!(
            List::new(&[1, 2, 3, 4, 5]).has_subsequence(&List::new(&[4, 3, 2])),
            false
        );
        assert_eq!(
            List::new(&[1, 2, 3, 4, 5]).has_subsequence(&List::new(&[2, 3, 4])),
            true
        );
    }
}

use std::rc::Rc;

pub mod my_program {

    pub fn abs(n: i32) -> u32 {
        if n < 0 {
            (-1 * n) as u32
        } else {
            n as u32
        }
    }

    fn format_abs(x: i32) -> String {
        format!("The absolute value of %d is {}", abs(x))
    }

    pub fn print_abs() -> () {
        print!("{}", format_abs(-42));
    }

    pub fn factorial(n: u32) -> u32 {
        let mut result = 1;
        for i in 1..=n {
            result *= i;
        }
        result
    }

    fn format_factorial(n: u32) -> String {
        format!("The factorial of {} is {}.", n, factorial(n))
    }

    pub fn print_abs_and_factorial() -> () {
        println!("{}", format_abs(-42));
        println!("{}", format_factorial(7));
    }

    pub fn format_result(name: &str, n: u32, f: fn(u32) -> u32) -> String {
        format!("The {} of {} is {}.", name, n, f(n))
    }

    pub fn find_first(ss: Vec<&str>, key: &str) -> i32 {
        for (index, element) in ss.iter().enumerate() {
            if element.eq(&key) {
                return index as i32;
            }
        }
        -1
    }

    pub fn find_first_<A>(as_: Vec<&A>, gt: fn(&A) -> bool) -> i32 {
        for (index, element) in as_.iter().enumerate() {
            if gt(element) {
                return index as i32;
            }
        }
        -1
    }
}

pub fn fib(n: u32) -> u32 {
    let mut current = 0;
    let mut next = 1;
    for _ in 0..n {
        let accumulator = current;
        current = next;
        next = accumulator + next;
    }
    current
}

pub fn is_sorted<A>(as_: &Vec<A>, gt: fn(&A, &A) -> bool) -> bool {
    for element in as_.windows(2) {
        match element {
            [current, next] if gt(current, next) => return false,
            _ => {}
        }
    }
    true
}

pub fn curry<'a: 'b, 'b, A, B, C, F>(
    f: F,
) -> Box<dyn Fn(&'a A) -> Box<dyn Fn(&'b B) -> C + 'b> + 'a>
where
    F: Fn(&A, &B) -> C + 'static,
    A: 'a,
    B: 'b,
    C: 'static,
{
    let f = Rc::new(f);
    Box::new(move |a: &'a A| {
        let f = Rc::clone(&f);
        Box::new(move |b: &'b B| f(a, b))
    })
}

pub fn uncurry<'a, 'b, A, B, C, F>(f: F) -> Box<dyn Fn(&'a A, &'b B) -> C + 'a>
where
    F: Fn(&'a A) -> Box<dyn Fn(&'b B) -> C + 'b> + 'a,
    A: 'a,
    B: 'b,
    C: 'static,
{
    Box::new(move |a: &'a A, b: &'b B| f(a)(b))
}

pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(&A) -> C
where
    A: ?Sized,
    F: Fn(&B) -> C,
    G: Fn(&A) -> B,
{
    move |a: &A| f(&g(a))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exercise21() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(6), 8);
        assert_eq!(fib(7), 13);
        assert_eq!(fib(8), 21);
        assert_eq!(fib(9), 34);
        // assert!(panic::catch_unwind(|| fib(-42)).is_err()); // Syntax Error
    }

    #[test]
    fn test_exercise22() {
        assert_eq!(
            is_sorted(&vec![1, 2, 3], |current, next| current > next),
            true
        );
        assert_eq!(
            is_sorted(&vec![1, 2, 1], |current, next| current > next),
            false
        );
        assert_eq!(
            is_sorted(&vec![3, 2, 1], |current, next| current < next),
            true
        );
        assert_eq!(
            is_sorted(&vec![1, 2, 3], |current, next| current < next),
            false
        );
    }

    #[test]
    fn test_exercise23() {
        let func = |a: &i32, b: &i32| a + b;
        assert_eq!(func(&42, &42), curry(func)(&42)(&42))
    }

    #[test]
    fn test_exercise24() {
        let func = |a: &'static i32| -> Box<dyn Fn(&'static i32) -> i32> {
            Box::new(move |b: &'static i32| *a + *b)
        };
        assert_eq!(func(&42)(&42), uncurry(func)(&42, &42));
    }

    #[test]
    fn test_exercise25() {
        let f = |x: &i32| 42 * x;
        let g = |x: &str| 42 * x.parse::<i32>().unwrap();
        assert_eq!((42 * 42 * 42), compose(f, g)(&"42"));
    }
}

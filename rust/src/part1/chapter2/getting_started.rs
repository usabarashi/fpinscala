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
}

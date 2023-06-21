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

    pub fn fuctorial(n: u32) -> u32 {
        let mut result = 1;
        for i in 1..=n {
            result *= i;
        }
        result
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exercise21() {
        assert_eq!(my_program::fib(0), 0);
        assert_eq!(my_program::fib(0), 0);
        assert_eq!(my_program::fib(1), 1);
        assert_eq!(my_program::fib(2), 1);
        assert_eq!(my_program::fib(3), 2);
        assert_eq!(my_program::fib(4), 3);
        assert_eq!(my_program::fib(5), 5);
        assert_eq!(my_program::fib(6), 8);
        assert_eq!(my_program::fib(7), 13);
        assert_eq!(my_program::fib(8), 21);
        assert_eq!(my_program::fib(9), 34);
        // assert!(panic::catch_unwind(|| fib(-42)).is_err()); // Syntax Error
    }
}

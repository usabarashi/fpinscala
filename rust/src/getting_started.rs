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
        /*
        fn go(n: u32, acc: u32) -> u32 {
            if n <= 0 {
                acc
            } else {
                go(n - 1, n * acc)
            }
        }
        go(n, 1)
        */

        let mut result = 1;
        for i in 1..=n {
            result *= i;
        }
        result
    }

    pub fn fib(n: u32) -> u32 {
        /*
        match n {
            0 => 0,
            1 => 1,
            _  => fib(n - 2) + fib(n - 1),
        }
        */

        /*
        fn go(number: u32, current: u32, next: u32) -> u32 {
            if number <= 0 {
                current
            } else {
                go(number - 1, next, current + next)
            }
        }
        go(n, 0, 1)
        */

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

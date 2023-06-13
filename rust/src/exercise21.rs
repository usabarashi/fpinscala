pub fn fib(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ if 1 < n => fib(n - 2) + fib(n - 1),
        _ => panic!("{}", n),
    }
}

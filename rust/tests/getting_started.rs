use fpinscala::getting_started::my_program::fib;

#[test]
fn test_exersize21() {
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

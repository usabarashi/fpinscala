fib :: Int -> Int
fib n
    | n == 0 = 0
    | n == 1 = 1
    | 1 < n = (fib (n - 2)) + (fib (n - 1))
    | otherwise = error "Invalid Numer."

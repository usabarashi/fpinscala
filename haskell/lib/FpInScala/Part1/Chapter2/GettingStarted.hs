module FpInScala.Part1.Chapter2.GettingStarted (abs', printAbs, factorial, fib) where

abs' :: Int -> Int
abs' n = if n < 0 then (-n) else n

formatAbs :: Int -> String
formatAbs x =
    "The absolute value of " ++ (show x) ++ " is " ++ (show (abs' x))

printAbs :: IO ()
printAbs =  putStrLn (formatAbs (-42))

factorial :: Int -> Int
factorial n
    | n < 0 = error $ "Invalid number: " ++ (show n)
    | n == 0 = 1
    | otherwise = n * factorial (n - 1)

fib :: Int -> Int
fib n
    | n < 0 = error $ "Invalid number: " ++ (show n)
    | n == 0 = 0
    | n == 1 = 1
    | otherwise = (fib (n - 2)) + (fib (n - 1))

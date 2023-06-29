module FpInScala.Part1.Chapter2.GettingStarted (abs', printAbs, factorial, fib, printAbsAndFactorial, formatResult, isSorted) where

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

formatFactorial :: Int -> String
formatFactorial n = "The factorial of " ++ (show n) ++ " is " ++ (show (factorial n)) ++ "."

printAbsAndFactorial :: IO ()
printAbsAndFactorial = do
    putStrLn (show (formatAbs (-42)))
    putStrLn (show (formatFactorial 7))

formatResult :: String -> Int -> (Int -> Int) -> String
formatResult name n f = "The " ++ name ++ " of " ++ (show n) ++ " is " ++ (show (f n)) ++ "."

findFirst :: [String] -> String -> Int
findFirst list target = loop (zip [0..] list) target
    where
        loop [] _ = -1
        loop ((index, value):rest) target'
            | value == target'   = index
            | otherwise         = loop rest target'

findFirst' :: [a] -> (a -> Bool) -> Bool
findFirst' list f = loop list f
    where
        loop [] _ = False
        loop (value:rest) f'
            | f' value == True  = True
            | otherwise         = loop rest f'

fib :: Int -> Int
fib n
    | n < 0 = error $ "Invalid number: " ++ (show n)
    | n == 0 = 0
    | n == 1 = 1
    | otherwise = (fib (n - 2)) + (fib (n - 1))

isSorted :: [a] -> (a -> a -> Bool) -> Bool
isSorted [] _                   = True
isSorted [_] _                  = True
isSorted (current:next:rest) f
    | f current next == True = False
    | otherwise              = isSorted (next:rest) f

curry :: (a -> b -> c) -> a -> (b -> c)
curry f a b = f a b

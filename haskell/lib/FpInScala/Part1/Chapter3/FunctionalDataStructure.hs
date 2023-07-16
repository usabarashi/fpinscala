module FpInScala.Part1.Chapter3.FunctionalDataStructure (List(Nil, Cons), sumList, list) where

data List a = Nil | Cons a (List a)

sumList :: List Int -> Int
sumList Nil = 0
sumList (Cons x xs) = x + sumList xs

productList :: List Double -> Double
productList Nil = 1.0
productList (Cons 0.0 _) = 0.0
productList (Cons x xs) = x * productList xs

list :: [a] -> List a
list [] = Nil
list (x:xs) = Cons x (list xs)
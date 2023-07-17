module FpInScala.Part1.Chapter3.FunctionalDataStructure
  ( List(Nil, Cons)
  , sumList
  , list
  , tail'
  , setHead
  , drop'
  , dropWhile'
  , init'
  , foldRight
  , length'
  , foldLeft
  , sumLeft
  , productLeft
  , lengthLeft
  , reverse'
  ) where

data List a = Nil | Cons a (List a)

instance Show a => Show (List a) where
  show Nil = "Nil"
  show (Cons x xs) = "Cons " ++ show x ++ " (" ++ show xs ++ ")"

instance Eq a => Eq (List a) where
  Nil == Nil = True
  (Cons x xs) == (Cons y ys) = x == y && xs == ys
  _ == _ = False

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

tail' :: List a -> List a
tail' Nil = error "Nil"
tail' (Cons _ xs) = xs

setHead :: List a -> a -> List a
setHead Nil _ = error "Nil"
setHead (Cons _ xs) x = Cons x xs

drop' :: List a -> Int -> List a
drop' Nil _ = Nil
drop' list'@(Cons _ xs) n
  | n <= 0 = list'
  | otherwise = drop' xs (n - 1)

dropWhile' :: List a -> (a -> Bool) -> List a
dropWhile' Nil _ = Nil
dropWhile' list'@(Cons x xs) f
  | f x = dropWhile' xs f
  | otherwise = list'

init' :: List a -> List a
init' Nil = error "Nil"
init' (Cons x Nil) = Nil
init' (Cons x xs) = Cons x (init' xs)

foldRight :: List a -> b -> (a -> b -> b) -> b
foldRight Nil b _ = b
foldRight (Cons x xs) b f = f x (foldRight xs b f)

length' :: List a -> Int
length' xs = foldRight xs 0 (\a b -> b + 1)

foldLeft :: List a -> b -> (b -> a -> b) -> b
foldLeft Nil b _ = b
foldLeft (Cons x xs) b f = foldLeft xs (f b x) f

sumLeft :: List Int -> Int
sumLeft xs = foldLeft xs 0 (\a x -> a + x)

productLeft :: List Double -> Double
productLeft xs = foldLeft xs 1.0 (\a x -> a * x)

lengthLeft :: List a -> Int
lengthLeft xs = foldLeft xs 0 (\a _ -> a + 1)

reverse' :: List a -> List a
reverse' xs = foldLeft xs Nil (\a head -> Cons head a)

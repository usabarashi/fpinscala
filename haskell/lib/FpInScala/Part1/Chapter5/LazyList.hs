module FpInScala.Part1.Chapter5.LazyList
  ( LazyList(Empty, Cons)
  , lazyList
  , headOption
  , toList
  ) where

data LazyList a = Empty | Cons a (LazyList a)
  deriving (Eq, Show)

lazyList :: [a] -> LazyList a
lazyList [] = Empty
lazyList (x:xs) = Cons x (lazyList xs)

headOption :: LazyList a -> Maybe a
headOption Empty = Nothing
headOption (Cons x xs) = Just x

toList :: LazyList a -> [a]
toList ll = go ll []
  where
    go :: LazyList a -> [a] -> [a]
    go Empty acc = reverse acc
    go (Cons h t) acc = go t (h : acc)

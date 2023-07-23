module FpInScala.Part1.Chapter3.Tree
  ( Tree(Leaf, Branch)
  , size
  , maximum'
  , depth
  ) where

data Tree a = Leaf a | Branch (Tree a) (Tree a)

size :: Tree a -> Int
size (Leaf x) = 1
size (Branch l r) = 1 + (size l) + (size r)

maximum' :: Tree Int -> Int
maximum' (Leaf x) = x
maximum' (Branch l r) = max (maximum' l) (maximum' r)

depth :: Tree a -> Int
depth (Leaf _) = 0
depth (Branch l r) = 1 + (max (depth l) (depth r))

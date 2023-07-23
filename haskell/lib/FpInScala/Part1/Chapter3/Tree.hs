module FpInScala.Part1.Chapter3.Tree
  ( Tree(Leaf, Branch)
  , size
  , maximum'
  , depth
  , map'
  ) where

data Tree a = Leaf a | Branch (Tree a) (Tree a)
  deriving Show

instance Eq a => Eq (Tree a) where
  (Leaf x) == (Leaf y) = x == y
  (Branch left1 right1) == (Branch left2 right2) = left1 == left2 && right1 == right2
  _ == _ = False

size :: Tree a -> Int
size (Leaf x) = 1
size (Branch l r) = 1 + (size l) + (size r)

maximum' :: Tree Int -> Int
maximum' (Leaf x) = x
maximum' (Branch l r) = max (maximum' l) (maximum' r)

depth :: Tree a -> Int
depth (Leaf _) = 0
depth (Branch l r) = 1 + (max (depth l) (depth r))

map' :: Tree a -> (a -> b) -> Tree b
map' (Leaf a) f = Leaf (f a)
map' (Branch l r) f = Branch (map' l f) (map' r f)

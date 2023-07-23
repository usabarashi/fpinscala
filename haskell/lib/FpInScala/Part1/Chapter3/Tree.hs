module FpInScala.Part1.Chapter3.Tree
  ( Tree(Leaf, Branch)
  , size
  , maximum'
  , depth
  , map'
  , sizeViaFold
  , depthViaFold
  , mapViaFold
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

fold :: Tree a -> (a -> b) -> (b -> b -> b) -> b
fold (Leaf value) f _ = f value
fold (Branch left right) f g = g (fold left f g) (fold right f g)

sizeViaFold :: Tree a -> Int
sizeViaFold t = fold t (\_ -> 1) (\l r -> 1 + l + r)

depthViaFold :: Tree a -> Int
depthViaFold t =  fold t (\_ -> 0) (\left right -> 1 + (max left right))

mapViaFold :: Tree a -> (a -> b) -> Tree b
mapViaFold t f = fold t (\value -> Leaf (f value)) (\left right -> Branch left right)

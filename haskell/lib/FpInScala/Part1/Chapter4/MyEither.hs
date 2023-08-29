module FpInScala.Part1.Chapter4.MyEither
  ( MyEither(MyLeft, MyRight)
  , map'
  , flatMap'
  , orElse
  , map2
  ) where

data MyEither l r = MyLeft l | MyRight r
  deriving (Eq, Show)

map' :: MyEither l r -> (r -> rr) -> MyEither l rr
map' (MyLeft l) _ = MyLeft l
map' (MyRight r) f = MyRight (f r)

flatMap' :: MyEither l r -> (r -> MyEither l rr) -> MyEither l rr
flatMap' (MyLeft l) _ = MyLeft l
flatMap' (MyRight r) f = f r

orElse :: MyEither l r -> (MyEither ll r) -> MyEither ll r
orElse (MyLeft _) f = f
orElse (MyRight r) _ = MyRight r

map2 :: MyEither l r1 -> MyEither l r2 -> (r1 -> r2 -> r3) -> MyEither l r3
map2 (MyLeft l) _ _ = MyLeft l
map2 _ (MyLeft l) _ = MyLeft l
map2 (MyRight r1) (MyRight r2) f = MyRight (f r1 r2)

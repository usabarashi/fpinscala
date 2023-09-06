module FpInScala.Part1.Chapter4.Validated
  ( Validated(Invalid, Valid)
  , toEither
  , map'
  , map2
  , fromEither
  , traverse'
  , sequence'
  ) where

data Validated e a = Invalid [e] | Valid a
  deriving (Eq, Show)

toEither :: Validated e a -> Either [e] a
toEither (Invalid es) = Left es
toEither (Valid a) = Right a

map' :: Validated e a -> (a -> b) -> Validated e b
map' (Invalid es) _ = Invalid es
map' (Valid a) f = Valid (f a)

map2 :: Validated e a -> Validated e b -> (a -> b -> c) -> Validated e c
map2 (Valid a) (Valid b) f = Valid (f a b)
map2 (Invalid es) _ _ = Invalid es
map2 _ (Invalid es) _ = Invalid es
map2 (Invalid es1) (Invalid es2) _ = Invalid (es1 ++ es2)

fromEither :: Either [e] a -> Validated e a
fromEither (Right a) = Valid a
fromEither (Left es) = Invalid es

traverse' :: (a -> Validated e b) -> [a] -> Validated e [b]
traverse' f as = foldr go (Valid []) as
  where
    go a acc = case (f a, acc) of
        (Invalid e1, Invalid e2) -> Invalid (e1 ++ e2)
        (Invalid e1, _) -> Invalid e1
        (_, Invalid e2) -> Invalid e2
        (Valid x, Valid xs) -> Valid (x : xs)

sequence' :: [Validated e a] ->  Validated e [a]
sequence' vs = traverse' (\x -> x) vs

module FpInScala.Part1.Chapter4.Option
  ( Option(None, Some)
  , map'
  , flatMap'
  , getOrElse
  , orElse
  , filter'
  , variance
  , map2
  , sequence'
  , sequenceFromTraverse
  ) where

data Option a = None | Some a
  deriving (Eq, Show)

map' :: Option a -> (a -> b) -> Option b
map' None _ = None
map' (Some a) f = Some (f a)

flatMap' :: Option a -> (a -> Option b) -> Option b
flatMap' a f = getOrElse (map' a f) None

-- def getOrElse[B >: A](default: => B): B =
getOrElse :: Option a -> a -> a
getOrElse None a = a
getOrElse (Some a) _ = a

-- def orElse[B >: A](ob: => Option[B]): Option[B] =
orElse :: Option a -> Option a -> Option a
orElse a f = getOrElse (map' a (\x -> Some x)) f

filter' :: Option a -> (a -> Bool) -> Option a
filter' a f = flatMap' a (\x -> if f(x) then Some x else None)

mean :: [Double] -> Option Double
mean [] = None
mean xs = Some (sum xs / fromIntegral (length xs))

variance :: [Double] -> Option Double
variance [] = None
variance xs = flatMap' (mean xs) (\m -> mean (map (\x -> (x - m) ^ 2) xs))

map2 :: Option a -> Option b -> (a -> b -> c) -> Option c
map2 a b f = flatMap' a (\a' -> map' b (\b' -> f a' b'))

sequence' :: [Option a] -> Option [a]
sequence' [] = Some []
sequence' (h:t) = flatMap' h (\hh -> map' (sequence' t) (\tt -> hh:tt))

traverse' :: [a] -> (a -> Option b) -> Option [b]
traverse' [] _ = Some []
traverse' (h:t) f = map2 (f h) (traverse' t f) (\hh -> \tt -> hh:tt)

sequenceFromTraverse :: [Option a] -> Option [a]
sequenceFromTraverse xs = traverse' xs (\x -> x)

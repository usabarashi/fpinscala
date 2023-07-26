module FpInScala.Part1.Chapter4.Option
  ( Option(None, Some)
  , map'
  , flatMap'
  , getOrElse
  , orElse
  , filter'
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

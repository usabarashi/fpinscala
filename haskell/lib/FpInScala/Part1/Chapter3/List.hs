module FpInScala.Part1.Chapter3.List
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
  , foldLeftFromRight
  , foldRightFromLeft
  , appendRight
  , concat'
  , incrementEach
  , doubleToString
  , map'
  , filter'
  , flatMap
  , filterFromFlatMap
  , addPairwise
  , zipWith'
  , take'
  , takeWhile'
  , forall
  , exists
  , scanLeft
  , scanRight
  , hasSubsequence
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

append :: List a -> List a -> List a
append Nil a2 = a2
append (Cons h t) a2 = Cons h (append t a2)

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

foldLeftFromRight :: List a -> b -> (b -> a -> b) -> b
foldLeftFromRight xs b f = foldRight xs b (\b h -> f h b)

foldRightFromLeft :: List a -> b -> (a -> b -> b) -> b
foldRightFromLeft xs b f = foldLeft xs b (\h b -> f b h)

appendRight :: List a ->  List a -> List a
appendRight a1 a2 = foldRight a1 a2 (\h b -> Cons h b)

concat' :: List (List a) -> List a
concat' xss = foldRight xss Nil append

incrementEach :: List Int -> List Int
incrementEach Nil = Nil
incrementEach xs = foldRight xs Nil (\h acc -> Cons (h + 1) acc)

doubleToString :: List Double -> List String
doubleToString xs = foldRight xs Nil (\h acc -> Cons (show h) acc)

map' :: List a -> (a -> b) ->List b
map' xs f = foldRight xs Nil (\h acc -> Cons (f h) acc)

filter' :: List a -> (a -> Bool) -> List a
filter' xs f = foldRight xs Nil (\h acc -> if f h then Cons h acc else acc)

flatMap :: List a -> (a -> List b) -> List b
flatMap xs f = concat' (map' xs f)

filterFromFlatMap :: List a -> (a -> Bool) -> List a
filterFromFlatMap xs f = flatMap xs  (\x -> if f x then list [x] else list [] )

addPairwise :: List Int -> List Int -> List Int
addPairwise Nil _ = Nil
addPairwise _ Nil = Nil
addPairwise (Cons a1 as1) (Cons a2 as2) = Cons (a1 + a2) (addPairwise as1 as2)

zipWith' :: List a -> List b -> (a -> b -> c) -> List c
zipWith' Nil _ _ = Nil
zipWith' _ Nil _ = Nil
zipWith' (Cons a as) (Cons b bs) f = Cons (f a b) (zipWith' as bs f)

take' :: List a -> Int -> List a
take' Nil _ = Nil
take' (Cons x xs) n
  | n <= 0 = Nil
  | otherwise = Cons x (take' xs (n - 1))

takeWhile' :: List a -> (a -> Bool) -> List a
takeWhile' Nil _ = Nil
takeWhile' (Cons x xs) f
  | f x == False = Nil
  | otherwise = Cons x (takeWhile' xs f)

forall :: List a -> (a -> Bool) -> Bool
forall (Cons x xs) f = foldLeft xs True (\acc h -> acc && (f h))

exists :: List a -> (a -> Bool) -> Bool
exists xs f = foldLeft xs False (\acc h -> acc || (f h))

scanLeft :: List a -> b -> (b -> a -> b) -> List b
scanLeft Nil acc _ = Cons acc Nil
scanLeft (Cons x xs) acc f = Cons acc (scanLeft xs (f acc x) f)

scanRight :: List a -> b -> (a -> b -> b) -> List b
scanRight Nil acc _ = Cons acc Nil
scanRight (Cons head tail) acc f =
  let
    newTail@(Cons accHead _) = scanRight tail acc f
    newHead = f head accHead
  in
  Cons newHead newTail

startWith :: Eq a => List a -> List a -> Bool
startWith _ Nil = True
startWith (Cons x1 xs1) (Cons x2 xs2)
  | x1 == x2 = startWith xs1 xs2
  | otherwise = False

hasSubsequence :: Eq a => List a -> List a -> Bool
hasSubsequence Nil sub = sub == Nil
hasSubsequence sup sub | startWith sup sub = True
hasSubsequence (Cons x xs) sub = hasSubsequence xs sub

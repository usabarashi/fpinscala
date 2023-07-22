module FpInScalaSpec.Part1.Chapter3.FunctionalDataStructureSpec where

import Control.Exception (evaluate)
import Test.Hspec

import FpInScala.Part1.Chapter3.FunctionalDataStructure
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
  )

exercise31Spec :: Spec
exercise31Spec = do
    let result :: Int
        result = case list [1, 2, 3, 4, 5] of
                    Cons x (Cons 2 (Cons 4 _)) -> x
                    Nil -> 42
                    Cons x (Cons y (Cons 3 (Cons 4 _))) -> x + y
                    Cons h t -> h + sumList t
                    _ -> 101
    describe "List" $ do
        it "Pattern matching" $
            result `shouldBe` 3

exercise32Spec :: Spec
exercise32Spec = do
    describe "List" $ do
        it "Nil tail" $ do
            evaluate (tail' (list [])) `shouldThrow` anyException
        it "Cons tail" $
            tail' (list [1, 2, 3]) `shouldBe` (list [2, 3])

exercise33Spec :: Spec
exercise33Spec = do
    describe "List" $ do
        it "Nil tail" $ do
            evaluate (setHead (list []) 42) `shouldThrow` anyException
        it "Cons tail" $
            (setHead (list [1, 2, 3]) 42) `shouldBe` (list [42, 2, 3])

exercise34Spec :: Spec
exercise34Spec = do
    describe "List" $ do
        it "Nil drop" $ do
            (drop' (list []) 42) `shouldBe` (list ([] :: [Int]))
        it "Cons drop" $
            (drop' (list [1, 2, 3, 4, 5]) 3) `shouldBe` (list [4, 5])

exercise35Spec :: Spec
exercise35Spec = do
    describe "List" $ do
        it "Nil dropWhile" $ do
            (dropWhile' (list []) (\n -> n < 42)) `shouldBe` (list [])
        it "Cons dropWhile" $
            (dropWhile' (list [1, 2, 3, 4, 5]) (\n -> n < 4)) `shouldBe` (list [4, 5])

exercise36Spec :: Spec
exercise36Spec = do
    describe "List" $ do
        it "Nil init" $ do
            evaluate (init' (list [])) `shouldThrow` anyException
        it "Cons init" $
            (init' (list [1, 2, 3, 4, 5])) `shouldBe` (list [1, 2, 3, 4])

exercise38Spec :: Spec
exercise38Spec = do
    describe "List" $ do
        it "foldRight" $
            let xs :: List Int
                xs = list [1, 2, 3]
                b :: List Int
                b =  Nil
                f :: Int -> List Int -> List Int
                f head tail = Cons head tail
            in
            (foldRight xs b f) `shouldBe` (list [1, 2, 3])

exercise39Spec :: Spec
exercise39Spec = do
    describe "List" $ do
        it "Nil length'" $
            (length' (list [])) `shouldBe` 0
        it "Cons length'" $
            (length' (list [1, 2, 3])) `shouldBe` 3


exercise310Spec :: Spec
exercise310Spec = do
    describe "List" $ do
        it "foldLeft'" $
            (foldLeft (list [1, 2, 3]) 0 (\a b -> a + b)) `shouldBe` 6

exercise311Spec :: Spec
exercise311Spec = do
    describe "List" $ do
        it "sumLeft" $
            (sumLeft (list [1, 2, 3])) `shouldBe` 6
        it "productLeft" $
            (productLeft (list [1.0, 2.0, 3.0])) `shouldBe` 6.0
        it "lengthLeft" $
            (lengthLeft (list [1, 2, 3])) `shouldBe` 3

exercise312Spec :: Spec
exercise312Spec = do
    describe "List" $ do
        it "reverse" $
            (reverse' (list [1, 2, 3])) `shouldBe` (list [3, 2, 1])

exercise313Spec :: Spec
exercise313Spec = do
    describe "List" $ do
        it "foldLeft from foldRight" $
            (foldLeftFromRight (list [1, 2, 3]) 0 (\b a -> b + a)) `shouldBe` 6
        it "foldRight from foldLeft" $
            (foldRightFromLeft (list [1, 2, 3]) 0 (\a b -> a + b)) `shouldBe` 6

exercise314Spec :: Spec
exercise314Spec = do
    describe "List" $ do
        it "append from foldRight" $
            (appendRight (list [1, 2, 3]) (list [4, 5, 6])) `shouldBe` (list [1, 2, 3, 4, 5, 6])

exercise315Spec :: Spec
exercise315Spec = do
    describe "List" $ do
        it "concat" $
            let xs1 :: List Int
                xs1 = list [1, 2, 3]
                xs2 :: List Int
                xs2 = list [4, 5, 6]
            in
            (concat' (list [xs1, xs2])) `shouldBe` (list [1, 2, 3, 4, 5, 6])

exercise316Spec :: Spec
exercise316Spec = do
    describe "List" $ do
        it "incrementEach" $
            (incrementEach (list [1, 2, 3, 4, 5])) `shouldBe` (list [2, 3, 4, 5, 6])

exercise317Spec :: Spec
exercise317Spec = do
    describe "List" $ do
        it "doubleToString" $
            (doubleToString (list [1.0, 2.0, 3.0, 4.0, 5.0])) `shouldBe` (list ["1.0", "2.0", "3.0", "4.0", "5.0"])

exercise318Spec :: Spec
exercise318Spec = do
    describe "List" $ do
        it "map" $
            (map' (list [1, 2, 3, 4, 5]) (\x -> x * 2)) `shouldBe` (list [2, 4, 6, 8, 10])

exercise319Spec :: Spec
exercise319Spec = do
    describe "List" $ do
        it "filter'" $
            (filter' (list [1, 2, 3, 4, 5]) (\x -> x `mod` 2 /= 0)) `shouldBe` (list [1, 3, 5])

exercise320Spec :: Spec
exercise320Spec = do
    describe "List" $ do
        it "flatMap" $
            (flatMap (list [1, 2, 3]) (\x -> list [x, x])) `shouldBe` (list [1, 1, 2, 2, 3, 3])

exercise321Spec :: Spec
exercise321Spec = do
    describe "List" $ do
        it "filter from flatMap" $
            (filterFromFlatMap (list [1, 2, 3]) (\x -> x `mod` 2 /= 0)) `shouldBe` (list [1, 3])

exercise322Spec :: Spec
exercise322Spec = do
    describe "List" $ do
        it "addPairwise" $
            (addPairwise (list [1, 2, 3]) (list [4, 5, 6])) `shouldBe` (list [5, 7, 9])

exercise323Spec :: Spec
exercise323Spec = do
    describe "List" $ do
        it "zipWith" $
            (zipWith' (list [1, 2, 3]) (list [4, 5, 6]) (\a b -> a + b)) `shouldBe` (list [5, 7, 9])

listInTheStandardLibrarySpec :: Spec
listInTheStandardLibrarySpec = do
    describe "List" $ do
        it "take'" $
            (take' (list [1, 2, 3, 4, 5]) 3) `shouldBe` (list [1, 2, 3])
        it "takeWhile'" $
            (takeWhile' (list [1, 2, 3, 4, 5]) (\x -> x <= 3)) `shouldBe` (list [1, 2, 3])
        it "forall False" $
            (forall (list [1, 2, 3, 4, 5]) (\x -> x <= 3)) `shouldBe` False
        it "forall True" $
            (forall (list [1, 2, 3, 4, 5]) (\x -> x <= 42)) `shouldBe` True
        it "exists False" $
            (exists (list [1, 2, 3, 4, 5]) (\x -> x == 42)) `shouldBe` False
        it "exists True" $
            (exists (list [1, 2, 3, 4, 5]) (\x -> x == 3)) `shouldBe` True
        it "scanLeft" $
            (scanLeft (list "abcde") "" (\acc x -> acc ++ [x])) `shouldBe` (list ["", "a", "ab", "abc", "abcd", "abcde"])
        it "scanRight" $
            (scanRight (list "abcde") "" (\x acc -> [x] ++ acc)) `shouldBe` (list ["abcde", "bcde", "cde", "de", "e", ""])

exercise324Spec :: Spec
exercise324Spec = do
    describe "List" $ do
        it "hasSubsequence False" $
            (hasSubsequence (list [1, 2, 3, 4, 5]) (list [4, 3, 2])) `shouldBe` False
        it "hasSubsequence True" $
            (hasSubsequence (list [1, 2, 3, 4, 5]) (list [2, 3, 4])) `shouldBe` True

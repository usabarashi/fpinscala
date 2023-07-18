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
            evaluate (tail' (list ([] :: [Int]))) `shouldThrow` anyException
        it "Cons tail" $
            tail' (list ([1, 2, 3] :: [Int])) `shouldBe` (list [2, 3] :: List Int)

exercise33Spec :: Spec
exercise33Spec = do
    describe "List" $ do
        it "Nil tail" $ do
            evaluate (setHead (list ([] :: [Int])) 42) `shouldThrow` anyException
        it "Cons tail" $
            (setHead (list ([1, 2, 3] :: [Int])) 42) `shouldBe` (list [42, 2, 3] :: List Int)

exercise34Spec :: Spec
exercise34Spec = do
    describe "List" $ do
        it "Nil drop" $ do
            (drop' (list ([] :: [Int])) 42) `shouldBe` (list [] :: List Int)
        it "Cons drop" $
            (drop' (list ([1, 2, 3, 4, 5] :: [Int])) 3) `shouldBe` (list [4, 5] :: List Int)

exercise35Spec :: Spec
exercise35Spec = do
    describe "List" $ do
        it "Nil dropWhile" $ do
            (dropWhile' (list ([] :: [Int])) (\n -> n < 42)) `shouldBe` (list [] :: List Int)
        it "Cons dropWhile" $
            (dropWhile' (list ([1, 2, 3, 4, 5] :: [Int])) (\n -> n < 4)) `shouldBe` (list [4, 5] :: List Int)

exercise36Spec :: Spec
exercise36Spec = do
    describe "List" $ do
        it "Nil init" $ do
            evaluate (init' (list ([] :: [Int]))) `shouldThrow` anyException
        it "Cons init" $
            (init' (list ([1, 2, 3, 4, 5] :: [Int]))) `shouldBe` (list [1, 2, 3, 4] :: List Int)

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
            (foldRight xs b f) `shouldBe` (list ([1, 2, 3] :: [Int]))

exercise39Spec :: Spec
exercise39Spec = do
    describe "List" $ do
        it "Nil length'" $
            (length' (list ([] :: [Int]))) `shouldBe` 0
        it "Cons length'" $
            (length' (list ([1, 2, 3] :: [Int]))) `shouldBe` 3


exercise310Spec :: Spec
exercise310Spec = do
    describe "List" $ do
        it "foldLeft'" $
            (foldLeft (list ([1, 2, 3] :: [Int])) 0 (\a b -> a + b)) `shouldBe` 6

exercise311Spec :: Spec
exercise311Spec = do
    describe "List" $ do
        it "sumLeft" $
            (sumLeft (list ([1, 2, 3] :: [Int]))) `shouldBe` 6
        it "productLeft" $
            (productLeft (list ([1.0, 2.0, 3.0] :: [Double]))) `shouldBe` 6.0
        it "lengthLeft" $
            (lengthLeft (list ([1, 2, 3] :: [Int]))) `shouldBe` 3

exercise312Spec :: Spec
exercise312Spec = do
    describe "List" $ do
        it "reverse" $
            (reverse' (list ([1, 2, 3] :: [Int]))) `shouldBe` (list ([3, 2, 1] :: [Int]))

exercise313Spec :: Spec
exercise313Spec = do
    describe "List" $ do
        it "foldLeft from foldRight" $
            (foldLeftFromRight (list ([1, 2, 3] :: [Int])) 0 (\b a -> b + a)) `shouldBe` 6
        it "foldRight from foldLeft" $
            (foldRightFromLeft (list ([1, 2, 3] :: [Int])) 0 (\a b -> a + b)) `shouldBe` 6

exercise314Spec :: Spec
exercise314Spec = do
    describe "List" $ do
        it "append from foldRight" $
            (appendRight (list ([1, 2, 3] :: [Int])) (list ([4, 5, 6] :: [Int]))) `shouldBe` (list ([1, 2, 3, 4, 5, 6] :: [Int]))

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

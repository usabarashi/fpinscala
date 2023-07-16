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

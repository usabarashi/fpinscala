module FpInScalaSpec.Part1.Chapter3.FunctionalDataStructureSpec where

import Control.Exception (evaluate)
import Test.Hspec

import FpInScala.Part1.Chapter3.FunctionalDataStructure (List(Nil, Cons), sumList, list, tail')

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

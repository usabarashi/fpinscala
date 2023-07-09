module FpInScalaSpec.Part1.Chapter2.GettingStartedSpec where

import Control.Exception (evaluate)
import Test.Hspec

import FpInScala.Part1.Chapter2.GettingStarted (compose, curry', fib, isSorted, uncurry')

exercise21Spec :: Spec
exercise21Spec = do
    describe "Valid number" $ do
        it "case 0" $
            fib 0 `shouldBe` 0
        it "case 1" $
            fib 1 `shouldBe` 1
        it "case 2" $
            fib 2 `shouldBe` 1
        it "case 3" $
            fib 3 `shouldBe` 2
        it "case 4" $
            fib 4 `shouldBe` 3
        it "case 5" $
            fib 5 `shouldBe` 5
        it "case 6" $
            fib 6 `shouldBe` 8
        it "case 7" $
            fib 7 `shouldBe` 13
        it "case 8" $
            fib 8 `shouldBe` 21
        it "case 9" $
            fib 9 `shouldBe` 34

    describe "Invalid number" $ do
        it "case -42" $ do
            evaluate (fib (-42)) `shouldThrow` anyException

exercise22Spec :: Spec
exercise22Spec = do
    describe "Higher-order Function" $ do
        it "case [1, 2, 3] > True" $
            isSorted [1 :: Int, 2, 3] (\current next -> current > next) `shouldBe` True
        it "case [1, 2, 1] > False" $
            isSorted [1 :: Int, 2, 1] (\current next -> current > next) `shouldBe` False
        it "case [3, 2, 1] < True" $
            isSorted [3 :: Int, 2, 1] (\current next -> current < next) `shouldBe` True
        it "case [1, 2, 3] < False" $
            isSorted [1 :: Int, 2, 3] (\current next -> current < next) `shouldBe` False

exercise23Spec :: Spec
exercise23Spec = do
    let func :: (Int, Int) -> Int
        func (a, b) = a + b
    describe "Curry" $ do
        it "curry" $
            (curry' func 42 42) `shouldBe` (func (42, 42))

exercise24Spec :: Spec
exercise24Spec = do
    let func :: Int -> Int -> Int
        func a b = a + b
    describe "Uncurry" $ do
        it "uncurry" $
            (uncurry' func (42, 42)) `shouldBe` (func 42 42)

exercise25Spec :: Spec
exercise25Spec = do
    let f :: Int -> Int
        f x = 42 * x
        g :: String -> Int
        g x = 42 * (read x)
    describe "Compose" $ do
        it "compose" $
            (42 * 42 * 42) `shouldBe` (compose f g "42")

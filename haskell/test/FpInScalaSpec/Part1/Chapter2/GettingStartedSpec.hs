module FpInScalaSpec.Part1.Chapter2.GettingStartedSpec where

import Control.Exception (evaluate)
import Test.Hspec

import FpInScala.Part1.Chapter2.GettingStarted (fib)

spec :: Spec
spec = do
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

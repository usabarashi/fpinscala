module FpInScalaSpec.Part1.Chapter5.LazyListSpec where

import Control.Exception (evaluate)
import Test.Hspec

import FpInScala.Part1.Chapter5.LazyList
  ( LazyList(Empty, Cons)
  , lazyList
  , headOption
  , toList
  )

exercise51Spec :: Spec
exercise51Spec = do
    describe "LazyList" $ do
        it "lazyLizt Empty" $
            (lazyList []) `shouldBe` (Empty :: LazyList Int)
        it "lazyLizt Cons" $
            (lazyList [42]) `shouldBe` (Cons 42 Empty :: LazyList Int)
        it "headOption Empty" $
            (headOption (lazyList [])) `shouldBe` (Nothing :: Maybe Int)
        it "headOption Cons" $
            (headOption (lazyList [42])) `shouldBe` (Just 42)
        it "toList" $
            (toList (lazyList [1, 2, 3])) `shouldBe` [1, 2, 3]

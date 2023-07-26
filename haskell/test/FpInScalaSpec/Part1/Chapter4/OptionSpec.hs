module FpInScalaSpec.Part1.Chapter4.OptionSpec where

import Control.Exception (evaluate)
import Test.Hspec

import FpInScala.Part1.Chapter4.Option
  ( Option(None, Some)
  , map'
  , flatMap'
  , getOrElse
  , orElse
  , filter'
  )

exercise41Spec :: Spec
exercise41Spec = do
    describe "Option" $ do
        it "map' None" $
            (map' None (\x -> x)) `shouldBe` (None :: Option Int)
        it "map' Some" $
            (map' (Some 42) (\x -> x)) `shouldBe` (Some 42)
        it "flatMap' None" $
            (flatMap' None (\x -> Some x)) `shouldBe` (None :: Option Int)
        it "flatMap' Some" $
            (flatMap' (Some 42) (\x -> Some x)) `shouldBe` (Some 42)
        it "getOrElse None" $
            (getOrElse None 0) `shouldBe` 0
        it "getOrElse Some" $
            (getOrElse (Some 42) 0) `shouldBe` 42
        it "orElse None" $
            (orElse None (Some 42)) `shouldBe` (Some 42)
        it "orElse Some" $
            (orElse (Some 42) None) `shouldBe` (Some 42)
        it "filter' None" $
            (filter' None (\x -> x == 42)) `shouldBe` None
        it "filter' Some" $
            (filter' (Some 42) (\x -> x == 42)) `shouldBe` (Some 42)

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
  , variance
  , map2
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

exercise42Spec :: Spec
exercise42Spec = do
    describe "Option" $ do
        it "variance" $
            (variance [1.0, 2.0, 3.0, 4.0, 5.0]) `shouldBe` (Some 2.0)

exercise43Spec :: Spec
exercise43Spec = do
    describe "Option" $ do
        it "map2 None" $
            (map2 (Some 42) None (\a -> \b -> a + b)) `shouldBe` (None :: Option Int)
        it "map2 Some" $
            (map2 (Some 42) (Some 42) (\a -> \b -> a + b)) `shouldBe` (Some 84)

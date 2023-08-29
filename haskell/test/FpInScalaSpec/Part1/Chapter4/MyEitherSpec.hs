module FpInScalaSpec.Part1.Chapter4.MyEitherSpec where

import Control.Exception (evaluate)
import Test.Hspec

import FpInScala.Part1.Chapter4.MyEither
  ( MyEither(MyLeft, MyRight)
  , map'
  , flatMap'
  , orElse
  , map2
  )

exercise46Spec :: Spec
exercise46Spec = do
    describe "MyEither" $ do
        it "map' Left" $
            (map' (MyLeft "error") (\x -> x)) `shouldBe` (MyLeft "error" :: MyEither String Int)
        it "map' Right" $
            (map' (MyRight 42) (\x -> x)) `shouldBe` (MyRight 42 :: MyEither String Int)
        it "flatMap' Left" $
            (flatMap' (MyLeft "error") (\x -> MyRight x)) `shouldBe` (MyLeft "error" :: MyEither String Int)
        it "flatMap' Right" $
            (flatMap' (MyRight 42) (\x -> MyRight x)) `shouldBe` (MyRight 42 :: MyEither String Int)
        it "orElse Lefte" $
            (orElse (MyLeft "error") (MyLeft "error")) `shouldBe` (MyLeft "error" :: MyEither String Int)
        it "orElse Right" $
            (orElse (MyRight 42) (MyLeft "error")) `shouldBe` (MyRight 42 :: MyEither String Int)
        it "map2 Left" $
            (map2 (MyLeft "error") (MyLeft "error") (\x -> \y -> x + y)) `shouldBe` (MyLeft "error" :: MyEither String Int)
        it "map2 Right" $
            (map2 (MyRight 42) (MyRight 42) (\x -> \y -> x + y)) `shouldBe` (MyRight 84 :: MyEither String Int)

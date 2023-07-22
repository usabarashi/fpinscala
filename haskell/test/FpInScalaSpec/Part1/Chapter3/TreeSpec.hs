module FpInScalaSpec.Part1.Chapter3.TreeSpec where

import Control.Exception (evaluate)
import Test.Hspec

import FpInScala.Part1.Chapter3.Tree
  ( Tree(Leaf, Branch)
  , maximum'
  )

exercise325Spec :: Spec
exercise325Spec = do
    describe "Tree" $ do
        it "maximum" $
            let
                tree = Branch (Branch (Leaf 1) (Leaf 2)) (Branch (Leaf 3) (Leaf 4))
            in
            (maximum' tree) `shouldBe` 4

module FpInScalaSpec.Part1.Chapter3.TreeSpec where

import Control.Exception (evaluate)
import Test.Hspec

import FpInScala.Part1.Chapter3.Tree
  ( Tree(Leaf, Branch)
  , maximum'
  , depth
  , map'
  , sizeViaFold
  , depthViaFold
  , mapViaFold
  )

exercise325Spec :: Spec
exercise325Spec = do
    describe "Tree" $ do
        it "maximum" $
            let
                tree = Branch (Branch (Leaf 1) (Leaf 2)) (Branch (Leaf 3) (Leaf 4))
            in
            (maximum' tree) `shouldBe` 4

exercise326Spec :: Spec
exercise326Spec = do
    describe "Tree" $ do
        it "depth" $
            let
                tree = Branch (Leaf 1) (Branch (Leaf 2) (Branch (Leaf 3) (Leaf 3)))
            in
            (depth tree) `shouldBe` 3

exercise327Spec :: Spec
exercise327Spec = do
    describe "Tree" $ do
        it "map'" $
            let
                tree = Branch (Leaf 1) (Branch (Leaf 2) (Branch (Leaf 3) (Leaf 3)))
                expect = Branch (Leaf "1") (Branch (Leaf "2") (Branch (Leaf "3") (Leaf "3")))
            in
            (map' tree (\x -> show x)) `shouldBe` expect

exercise328Spec :: Spec
exercise328Spec = do
    describe "Tree" $ do
        it "sizeViaFold" $
            let
                tree = Branch (Branch (Leaf 1) (Leaf 2)) (Branch (Leaf 3) (Leaf 4))
            in
            (sizeViaFold tree) `shouldBe` 7
        it "depthViaFold" $
            let
                tree = Branch (Leaf 1) (Branch (Leaf 2) (Branch (Leaf 3) (Leaf 3)))
            in
            (depthViaFold tree) `shouldBe` 3
        it "mapViaFold" $
            let
                tree = Branch (Leaf 1) (Branch (Leaf 2) (Branch (Leaf 3) (Leaf 3)))
                expect = Branch (Leaf "1") (Branch (Leaf "2") (Branch (Leaf "3") (Leaf "3")))
            in
            (mapViaFold tree (\x -> show x)) `shouldBe` expect

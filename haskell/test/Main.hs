module Main (main) where

import Test.Hspec
import qualified FpInScalaSpec.Part1.Chapter2.GettingStartedSpec

main :: IO ()
main = hspec $ do
    describe "Exercise 2.1" FpInScalaSpec.Part1.Chapter2.GettingStartedSpec.spec

module Main (main) where

import Test.Hspec
import qualified FpInScalaSpec.Part1.Chapter2.GettingStartedSpec

main :: IO ()
main = hspec $ do
    describe "Exercise 2.1" FpInScalaSpec.Part1.Chapter2.GettingStartedSpec.exercise21Spec
    describe "Exercise 2.2" FpInScalaSpec.Part1.Chapter2.GettingStartedSpec.exercise22Spec
    describe "Exercise 2.3" FpInScalaSpec.Part1.Chapter2.GettingStartedSpec.exercise23Spec
    describe "Exercise 2.4" FpInScalaSpec.Part1.Chapter2.GettingStartedSpec.exercise24Spec
    describe "Exercise 2.5" FpInScalaSpec.Part1.Chapter2.GettingStartedSpec.exercise25Spec

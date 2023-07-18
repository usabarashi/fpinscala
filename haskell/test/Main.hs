module Main (main) where

import Test.Hspec
import FpInScalaSpec.Part1.Chapter2.GettingStartedSpec
import FpInScalaSpec.Part1.Chapter3.FunctionalDataStructureSpec

main :: IO ()
main = hspec $ do
    describe "Exercise 2.1" FpInScalaSpec.Part1.Chapter2.GettingStartedSpec.exercise21Spec
    describe "Exercise 2.2" FpInScalaSpec.Part1.Chapter2.GettingStartedSpec.exercise22Spec
    describe "Exercise 2.3" FpInScalaSpec.Part1.Chapter2.GettingStartedSpec.exercise23Spec
    describe "Exercise 2.4" FpInScalaSpec.Part1.Chapter2.GettingStartedSpec.exercise24Spec
    describe "Exercise 2.5" FpInScalaSpec.Part1.Chapter2.GettingStartedSpec.exercise25Spec
    describe "Exercise 3.1" FpInScalaSpec.Part1.Chapter3.FunctionalDataStructureSpec.exercise31Spec
    describe "Exercise 3.2" FpInScalaSpec.Part1.Chapter3.FunctionalDataStructureSpec.exercise32Spec
    describe "Exercise 3.3" FpInScalaSpec.Part1.Chapter3.FunctionalDataStructureSpec.exercise33Spec
    describe "Exercise 3.4" FpInScalaSpec.Part1.Chapter3.FunctionalDataStructureSpec.exercise34Spec
    describe "Exercise 3.5" FpInScalaSpec.Part1.Chapter3.FunctionalDataStructureSpec.exercise35Spec
    describe "Exercise 3.6" FpInScalaSpec.Part1.Chapter3.FunctionalDataStructureSpec.exercise36Spec
    describe "Exercise 3.8" FpInScalaSpec.Part1.Chapter3.FunctionalDataStructureSpec.exercise38Spec
    describe "Exercise 3.9" FpInScalaSpec.Part1.Chapter3.FunctionalDataStructureSpec.exercise39Spec
    describe "Exercise 3.10" FpInScalaSpec.Part1.Chapter3.FunctionalDataStructureSpec.exercise310Spec
    describe "Exercise 3.11" FpInScalaSpec.Part1.Chapter3.FunctionalDataStructureSpec.exercise311Spec
    describe "Exercise 3.12" FpInScalaSpec.Part1.Chapter3.FunctionalDataStructureSpec.exercise312Spec
    describe "Exercise 3.13" FpInScalaSpec.Part1.Chapter3.FunctionalDataStructureSpec.exercise313Spec

module Main (main) where

import Test.Hspec
import FpInScalaSpec.Part1.Chapter2.GettingStartedSpec
import FpInScalaSpec.Part1.Chapter3.ListSpec
import FpInScalaSpec.Part1.Chapter3.TreeSpec

main :: IO ()
main = hspec $ do
    describe "Exercise 2.1" FpInScalaSpec.Part1.Chapter2.GettingStartedSpec.exercise21Spec
    describe "Exercise 2.2" FpInScalaSpec.Part1.Chapter2.GettingStartedSpec.exercise22Spec
    describe "Exercise 2.3" FpInScalaSpec.Part1.Chapter2.GettingStartedSpec.exercise23Spec
    describe "Exercise 2.4" FpInScalaSpec.Part1.Chapter2.GettingStartedSpec.exercise24Spec
    describe "Exercise 2.5" FpInScalaSpec.Part1.Chapter2.GettingStartedSpec.exercise25Spec
    describe "Exercise 3.1" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise31Spec
    describe "Exercise 3.2" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise32Spec
    describe "Exercise 3.3" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise33Spec
    describe "Exercise 3.4" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise34Spec
    describe "Exercise 3.5" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise35Spec
    describe "Exercise 3.6" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise36Spec
    describe "Exercise 3.8" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise38Spec
    describe "Exercise 3.9" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise39Spec
    describe "Exercise 3.10" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise310Spec
    describe "Exercise 3.11" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise311Spec
    describe "Exercise 3.12" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise312Spec
    describe "Exercise 3.13" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise313Spec
    describe "Exercise 3.14" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise314Spec
    describe "Exercise 3.15" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise315Spec
    describe "Exercise 3.16" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise316Spec
    describe "Exercise 3.17" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise317Spec
    describe "Exercise 3.18" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise318Spec
    describe "Exercise 3.19" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise319Spec
    describe "Exercise 3.20" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise320Spec
    describe "Exercise 3.21" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise321Spec
    describe "Exercise 3.22" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise322Spec
    describe "Exercise 3.23" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise323Spec
    describe "LISTS IN THE STANDARD LIBRARY" FpInScalaSpec.Part1.Chapter3.ListSpec.listInTheStandardLibrarySpec
    describe "Exercise 3.24" FpInScalaSpec.Part1.Chapter3.ListSpec.exercise324Spec
    describe "Exercise 3.25" FpInScalaSpec.Part1.Chapter3.TreeSpec.exercise325Spec
    describe "Exercise 3.26" FpInScalaSpec.Part1.Chapter3.TreeSpec.exercise326Spec
    describe "Exercise 3.27" FpInScalaSpec.Part1.Chapter3.TreeSpec.exercise327Spec

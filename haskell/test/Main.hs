module Main (main) where

import Test.Hspec
import qualified FpInScalaSpec.Exercise21Spec

main :: IO ()
main = hspec $ do
    describe "Exercise21" FpInScalaSpec.Exercise21Spec.spec

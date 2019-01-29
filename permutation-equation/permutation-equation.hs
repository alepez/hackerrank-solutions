import Data.List (elemIndex)
import Data.Maybe (fromMaybe)

permEq :: Int -> [Int] -> [Int]
permEq n ps = p . p <$> [1 .. n]
  where
    p x = 1 + fromMaybe 0 (elemIndex x ps)

main = do
  n <- read <$> getLine
  ps <- fmap read . words <$> getLine
  putStr $ unlines . fmap show $ permEq n ps

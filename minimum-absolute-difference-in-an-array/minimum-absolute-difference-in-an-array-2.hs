import Data.List

minAbsDiff xs = minimum diffs
  where
    absDiff x y = abs (x - y)
    diffs = zipWith absDiff (tail sorted) sorted
    sorted = sort xs

parseInput = (fmap read) . words . head . (drop 1) . lines

main = interact $ show . minAbsDiff . parseInput

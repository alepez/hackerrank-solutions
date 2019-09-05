import Data.Function (on)
import Data.List (minimumBy, sortOn, zipWith)

minIndexDiff :: [[Int]] -> Int
minIndexDiff lists = fst (minimumBy (compare `on` snd) diffs)
  where
    enumerateAndSort ls = sortOn fst (zip ls [0 ..])
    [xs, ys] = fmap enumerateAndSort lists
    diffs = zipWith (\(x, i) (_, j) -> (x, abs (i - j))) xs ys

main =
  interact
    (show . minIndexDiff . ((fmap . fmap) read) . fmap words . tail . lines)

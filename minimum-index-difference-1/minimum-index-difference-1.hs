import Data.Function (on)
import Data.List (minimumBy, sortOn, zipWith)

minIndexDiff :: [[Int]] -> Int
minIndexDiff [xs, ys] = fst (minimumBy (compare `on` snd) diffs)
  where
    enumerateAndSort ls = sortOn fst (zip ls [0 ..])
    xs' = enumerateAndSort xs
    ys' = enumerateAndSort ys
    diffs = zipWith (\(x, i) (_, j) -> (x, abs (i - j))) xs' ys'

main =
  interact
    (show . minIndexDiff . ((fmap . fmap) read) . fmap words . tail . lines)

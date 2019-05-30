import Data.List (group, maximumBy, sort)
import Data.Ord (comparing)

theMostFrequent :: [Int] -> Int
theMostFrequent = fst . maximumBy comp . fmap valueAndLength . group . sort
  where
    comp = comparing snd <> comparing (negate . fst)
    valueAndLength xs = (head xs, length xs)

main = interact $ show . theMostFrequent . fmap read . tail . words

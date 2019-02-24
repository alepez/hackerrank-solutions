import Data.List (groupBy, sortBy)
import Data.Function (on)
import Data.Ord (comparing)

safeMinimum [] = -1
safeMinimum xs = minimum xs

diffAdjacent xs = zipWith (-) (tail xs) xs

minDistance :: [Int] -> Int
minDistance = safeMinimum
            . concatMap diffAdjacent
            . (fmap . fmap) fst
            . groupBy ((==) `on` snd)
            . sortBy (comparing snd)
            . zip [0..]

main = do
  _:a <- fmap read . words <$> getContents
  print $ minDistance a

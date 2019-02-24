import Data.List (groupBy, sort)
import Data.Maybe

minimumMay [] = Nothing
minimumMay xs = Just $ minimum xs

minDistance :: [Int] -> Int
minDistance a = fromMaybe (-1) m
  where
    sorted = sort $ zip a [0..]
    groupedIndexes = fmap snd <$> groupBy sameValue sorted
    sameValue l r = fst l == fst r
    diff g = zipWith (-) (tail g) g
    m = minimumMay
      . fmap minimum
      . filter (not . null)
      . fmap diff
      $ groupedIndexes

main = do
  _:a <- fmap read . words <$> getContents
  print $ minDistance a

import Data.List (groupBy, sort)
import Data.Function (on)

minDistance :: [Int] -> Int
minDistance a =
  case ms of
    [] -> -1
    xs -> minimum xs
  where
    diff g = zipWith (-) (tail g) g
    ms = fmap minimum
       . filter (not . null)
       . fmap (diff . fmap snd)
       . groupBy ((==) `on` fst)
       . sort
       $ zip a [0..]

main = do
  _:a <- fmap read . words <$> getContents
  print $ minDistance a

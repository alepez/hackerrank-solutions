import Data.List (sortBy)

minimumCost :: Int -> Int -> [Int] -> Int
minimumCost _ _ [] = 0
minimumCost k m c = t + minimumCost k m' c'
  where
    m' = m + 1
    c' = drop k c
    t = sum $ (* m) <$> take k c

main = do
  _:k:c <- fmap read . words <$> getContents
  print $ minimumCost k 1 $ sortBy (flip compare) c

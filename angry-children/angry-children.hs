import Data.List (sort)

minUnfairness k xs = minimum $ zipWith (-) (drop (k - 1) xs) xs

main = do
  _:k:xs <- fmap read . lines <$> getContents
  print $ minUnfairness k (sort xs)

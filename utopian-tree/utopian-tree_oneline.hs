utopianTree :: Int -> Int
utopianTree 0 = 1
utopianTree n = ((2 ^ (((n - 1) `quot` 2) + 2)) - 2) + (fromEnum . even) n

main = do
  arr <- fmap read . tail . lines <$> getContents
  putStr $ concatMap ((++ "\n") . show) $ fmap utopianTree arr

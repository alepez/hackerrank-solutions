isBeautiful :: Int -> Int -> Bool
isBeautiful k n = (n - r) `mod` k == 0
  where
    r = read $ reverse $ show n

main = do
  [from, to, k] <- fmap read . words <$> getContents
  print $ length $ filter (isBeautiful k) [from..to]

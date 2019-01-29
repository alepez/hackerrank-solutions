circularArrayRotation :: [Int] -> Int -> [Int] -> [Int]
circularArrayRotation a k = fmap go
  where
    go m = a !! mod (m - k) l
    l = length a

main = do
  [_, k, _] <- fmap read . words <$> getLine
  a <- fmap read . words <$> getLine
  ms <- fmap read . lines <$> getContents
  putStr $ unlines . fmap show $ circularArrayRotation a k ms

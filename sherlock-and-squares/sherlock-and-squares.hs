sherlockAndSquares :: [Double] -> Int
sherlockAndSquares [a, b] = b' - a' + 1
  where
    a' = ceiling . sqrt $ a
    b' = floor . sqrt $ b

main =
  interact $
  unlines . fmap (show . sherlockAndSquares . fmap read . words) . tail . lines

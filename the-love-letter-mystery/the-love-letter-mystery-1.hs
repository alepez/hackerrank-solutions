solve :: String -> Int
solve s = sum $ zipWith diff l r
  where
    half = take (length s `div` 2)
    l = half s
    r = half (reverse s)
    diff x y = abs ((fromEnum x) - (fromEnum y))

main = interact $ unlines . fmap (show . solve) . tail . lines

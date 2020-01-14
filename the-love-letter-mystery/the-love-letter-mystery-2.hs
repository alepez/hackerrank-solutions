solve :: String -> Int
solve s = sum $ zipWith diff l r
  where
    xs = fromEnum <$> s
    half = take $ length s `div` 2
    l = half xs
    r = half (reverse xs)
    diff x y = abs $ x - y

main = interact $ unlines . fmap (show . solve) . tail . lines

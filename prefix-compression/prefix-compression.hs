prefixCompression :: [String] -> [String]
prefixCompression [x, y] = [p, x', y']
  where
    p = fmap fst $ takeWhile (\(a,b) -> a == b) $ zip x y
    x' = drop (length p) x
    y' = drop (length p) y

showStr s = unwords [ ( show . length ) s , s ]

main = interact (unlines . fmap showStr . prefixCompression . lines)

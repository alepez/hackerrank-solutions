main = interact $ ser . fmap compute . de
de = fmap ((\[x, y] -> (x, y)) . fmap read) . fmap words . drop 1 . lines
ser = unlines . fmap show

compute (l, b) = (l `quot` z) * (b `quot` z)
  where
    z = gcd l b

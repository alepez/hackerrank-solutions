main = interact $ ser . compute . de

de s = zip3 lows highs closes
  where
    [lows, highs, closes] = (fmap read) <$> words <$> (drop 1 $ lines s)

ser (gapUp, gapDown) = unwords . map show $ [gapUp, gapDown]

compute :: [(Int, Int, Int)] -> (Int, Int)
compute xs = (length $ gap up, length $ gap down)
  where
    lh = [(l, h) | (l, h, c) <- xs]
    c = [c | (l, h, c) <- xs]
    gap op = filter (uncurry op) $ zip (tail lh) c
    up (l, h) c = l > c
    down (l, h) c = h < c

main = interact $ ser . compute . de

de :: String -> [((Int, Int), Int)]
de s = zip (zip lows highs) closes
  where
    [lows, highs, closes] = (fmap read) <$> words <$> (drop 1 $ lines s)

ser :: (Int, Int) -> String
ser (gapUp, gapDown) = unwords . map show $ [gapUp, gapDown]

compute :: [((Int, Int), Int)] -> (Int, Int)
compute xs = (length gapUp, length gapDown)
  where
    gapDown = filter isGapLow z
    gapUp = filter isGapUp z
    z = zip (snd <$> xs) (fst <$> (tail xs))

isGapLow (pc, (_, h)) = h < pc
isGapUp (pc, (l, _)) = l > pc

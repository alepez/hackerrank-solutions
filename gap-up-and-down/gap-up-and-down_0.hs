main = interact $ ser . compute . de

de :: String -> [(Int, Int, Int)]
de s = zip3 lows highs closes
  where
    [lows, highs, closes] = fmap (fmap read) $ fmap words $ drop 1 $ lines s

ser :: (Int, Int) -> String
ser (gapUp, gapDown) = unwords . map show $ [gapUp, gapDown]

compute :: [(Int, Int, Int)] -> (Int, Int)
compute xs = (gapUpCount, gapLowCount)
  where
    gapLowCount = length . filter isGapLow $ z
    gapUpCount = length . filter isGapUp $ z
    z = zip (takeClose xs) (takeLowHigh $ tail xs)
    takeLowHigh xs = [(l, h) | (l, h, c) <- xs]
    takeClose xs = [c | (l, h, c) <- xs]

isGapLow (pc, (_, h)) = h < pc
isGapUp (pc, (l, _)) = l > pc

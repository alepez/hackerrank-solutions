import Text.Printf

distance :: ((Double, Double), (Double, Double)) -> Double
distance ((x0, y0), (x1, y1)) = sqrt ((x1 - x0) * (x1 - x0) + (y1 - y0) * (y1 - y0))

mostDistant :: [(Int, Int)] -> Double
mostDistant points = maximum . fmap distance $ pairs
  where
    hor = [x | (x, y) <- points, y == 0]
    ver = [y | (x, y) <- points, x == 0]
    maxHor = (fromIntegral $ maximum hor, 0.0)
    minHor = (fromIntegral $ minimum hor, 0.0)
    maxVer = (0.0, fromIntegral $ maximum ver)
    minVer = (0.0, fromIntegral $ minimum ver)
    pairs =
      [ (maxHor, minHor),
        (maxHor, maxVer),
        (maxHor, minVer),
        (minVer, maxVer),
        (minVer, minHor),
        (minHor, maxVer)
      ]

parsePoint :: String -> (Int, Int)
parsePoint s = (x, y)
  where
    [x, y] = read <$> words s

main = interact (printf "%.12f" . mostDistant . fmap parsePoint . tail . lines)


distance :: ((Double, Double), (Double, Double)) -> Double
distance ((x0, y0), (x1, y1)) = sqrt ((x1 - x0) * (x1 - x0) + (y1 - y0) * (y1 - y0))

mostDistant :: [(Double, Double)] -> Double
mostDistant points = maximum . fmap distance $ [(a, b) | a <- points, b <- points]

parsePoint :: String -> (Double, Double)
parsePoint s = (x, y)
    where
        [x, y] = read <$> words s

main = interact (show . mostDistant . fmap parsePoint . tail . lines)

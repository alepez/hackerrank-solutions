countElement :: Eq a => a -> [a] -> Int
countElement x = (length . filter (== x))

freqTable :: Eq a => [a] -> [a] -> [(a, Int)]
freqTable ys xs = filter ((> 0) . snd) $ fmap (\y -> (y, countElement y xs)) ys

specialSort :: [(Char, Int)] -> [(Char, Int)]
specialSort (('0', zn):(y, n):fs) = (y, 1) : ('0', zn) : (y, n - 1) : fs
specialSort fs = fs

srs :: String -> String
srs =
  (concatMap (\(y, n) -> replicate n y)) .
  specialSort . (freqTable ['0' .. '9'])

main = interact $ srs

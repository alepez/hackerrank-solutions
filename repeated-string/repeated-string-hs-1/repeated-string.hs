solve :: String -> Int -> Int
solve s n = rep * ( countA s ) + countA las
  where
    l = length s
    rep = div n l
    las = take ( rem n l ) s
    countA = length . filter (== 'a')

fromString :: String -> (String, Int)
fromString input = (s, read n)
  where
    [s, n] = lines input

main = interact $ show . uncurry solve . fromString

main = interact $ show . (+ 1) . length . filter (\x -> 'A' <= x && x <= 'Z')

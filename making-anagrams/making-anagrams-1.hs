parse :: String -> (String, String)
parse input = (a, b)
  where
    [a, b] = lines input

freq :: String -> [Int]
freq s = [length $ filter (== c) s | c <- ['a' .. 'z']]

solve :: String -> String -> Int
solve a b = sum $ abs <$> zipWith (-) (freq a) (freq b)

main = interact $ show . uncurry solve . parse

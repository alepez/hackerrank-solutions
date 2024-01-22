main = interact $ show . solve . parse

-- Parse the input
parse :: String -> ([Int], Int, Int)
parse input = (s, d, m)
  where
    [s, [d, m]] = fmap (fmap read) $ fmap words $ drop 1 $ lines input

-- Count segments where the sum of elements is d
solve :: ([Int], Int, Int) -> Int
solve (s, d, m) = length $ filter (== d) $ sum <$> segments m s

-- Return all contiguous segments of length n from the given list.
segments :: Int -> [a] -> [[a]]
segments _ [] = []
segments n xs = take n xs : segments n (drop 1 xs)


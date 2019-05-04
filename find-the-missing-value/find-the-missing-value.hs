findTheMissingValue :: [Integer] -> Integer
findTheMissingValue (n:xs) = (n * (n + 1) `div` 2) - sum xs
main = interact $ show . findTheMissingValue . fmap read . words

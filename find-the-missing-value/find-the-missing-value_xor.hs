import Data.Bits (xor)
findTheMissingValue :: [Integer] -> Integer
findTheMissingValue (n:xs) = foldl xor 0 ([1..n] ++ xs)
main = interact $ show . findTheMissingValue . fmap read . words

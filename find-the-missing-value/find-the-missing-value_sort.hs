import Data.List (sort)

findTheMissingValue (_:xs) = head [ i | (x, i) <- zip (sort xs) [1..], i /= x ]

main = interact $ show . findTheMissingValue . fmap read . words

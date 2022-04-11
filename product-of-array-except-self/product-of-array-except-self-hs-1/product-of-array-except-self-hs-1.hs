main = interact $ ser . solve . de

de :: String -> [Int]
de = fmap read . words . head . drop 1 . lines

ser :: [Int] -> String
ser = unwords . fmap show

solve :: [Int] -> [Int]
solve xs = zipWith (*) prefixProdFromRight prefixProdFromLeft
  where
    prefixProdFromLeft = init $ scanl (*) 1 xs
    prefixProdFromRight = tail $ reverse $ scanl (*) 1 (reverse xs)

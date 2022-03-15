import Data.List

main = interact $ ser . fmap compute . de

de :: String -> [(Int, [Int], [Int])]
de s = fmap readGroup $ groupN 3 $ drop 1 $ lines s

ser :: [Bool] -> String
ser = unlines . fmap yesNo

yesNo True = "YES"
yesNo False = "NO"

groupN :: Int -> [a] -> [[a]]
groupN _ [] = []
groupN n l = (take n l) : (groupN n (drop n l))

readGroup :: [String] -> (Int, [Int], [Int])
readGroup lines = (k, as, bs)
  where
    [_, k] = fmap read $ words $ head lines
    as = fmap read $ words $ head $ drop 1 $ lines
    bs = fmap read $ words $ head $ drop 2 $ lines

compute :: (Int, [Int], [Int]) -> Bool
compute (k, as, bs) =
  all (>= k) $ zipWith (+) (sort as) (reverse $ sort bs)

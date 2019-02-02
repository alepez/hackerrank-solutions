import Data.List.Split

numbers :: String -> [Int]
numbers b = read <$> words b

balancedSum :: [Int] -> Bool
balancedSum [_] = True
balancedSum arr = any match $ zip prefixSum $ tail prefixSum
  where
    match (v, v') = v' == s - v
    prefixSum = scanl1 (+) (0:arr)
    s = last prefixSum

output True = "YES"
output False = "NO"

main = do
  _ <- getLine
  testCases <- fmap (numbers . last) . chunksOf 2 . lines <$> getContents
  putStr . unlines $ output . balancedSum <$> testCases

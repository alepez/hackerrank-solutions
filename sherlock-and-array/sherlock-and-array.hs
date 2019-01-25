import Data.List.Split

numbers :: String -> [Int]
numbers b = read <$> words b

balancedSum [_] = True
balancedSum arr = found
  where
    (found, _, _) = foldl go (False, 0, sum $ tail arr) (zip arr $ tail arr)
    go (True, _, _) _ = (True, 0, 0)
    go (_, l, r) (v, w) = (l == r, l + v, r - w)

output True = "YES"
output False = "NO"

main = do
  _ <- getLine
  testCases <- fmap (numbers . last) . chunksOf 2 . lines <$> getContents
  putStr . unlines $ output . balancedSum <$> testCases

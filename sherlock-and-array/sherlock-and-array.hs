import Data.List.Split (chunksOf)

doSomething :: [Int] -> Bool
doSomething [_] = True
doSomething arr = any match $ zip prefixSum $ tail prefixSum
  where
    match (v, v') = v' == s - v
    prefixSum = scanl1 (+) (0 : arr)
    s = last prefixSum

yesNo True = "YES"
yesNo False = "NO"

main =
  getContents >>=
  putStr .
  unlines .
  fmap (yesNo . doSomething . fmap read . words . snd) .
  filter fst . zip (concat $ repeat [False, True]) . tail . lines

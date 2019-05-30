import Data.List (group, sort)

countPairs :: [Int] -> Int
countPairs =
  sum . fmap (\x -> x * (x - 1)) . filter (> 1) . fmap length . group . sort

main =
  interact $
  unlines .
  fmap (show . countPairs . fmap read . words . snd) .
  filter fst . zip (concat $ repeat [False, True]) . tail . lines

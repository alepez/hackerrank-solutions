import Data.HashMap.Strict (fromListWith, elems)

frequency :: [Int] -> [Int]
frequency xs = elems (fromListWith (+) [(x, 1) | x <- xs])

doSomething :: [Int] -> Int
doSomething =
  sum . fmap (\x -> x * (x - 1)) . filter (> 1) . frequency

main =
  interact $
  unlines .
  fmap (show . doSomething . fmap read . words . snd) .
  filter fst . zip (concat $ repeat [False, True]) . tail . lines

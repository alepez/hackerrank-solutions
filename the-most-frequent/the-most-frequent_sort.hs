import Data.List

theMostFrequent :: [Int] -> Int
theMostFrequent =
  head .
  fmap (head . fst) .
  sortOn (negate . snd) . fmap (\x -> (x, length x)) . group . sort

main = interact $ show . theMostFrequent . fmap read . tail . words

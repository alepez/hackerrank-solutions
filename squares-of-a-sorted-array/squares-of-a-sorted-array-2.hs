import Data.List

fromInput = fmap read . words . head . drop 1 . lines
toOutput = unwords


-- This function causes stack overflow due to recursion

f :: [Int] -> [Int]
f xs = sort_sorted_lists (reverse neg_sq) pos_sq
  where
    pos= filter (>=0) xs
    neg= filter (<0) xs
    to_squares = fmap (\x -> x * x)
    pos_sq = to_squares pos
    neg_sq = to_squares neg
    sort_sorted_lists [] [] = []
    sort_sorted_lists (x:[]) [] = [x]
    sort_sorted_lists [] (y:[]) = [y]
    sort_sorted_lists (x:[]) (y:[]) =
      if x < y
      then [x, y]
      else [y, x]
    sort_sorted_lists (x:xs) (y:ys) =
      if x < y
      then (x : (sort_sorted_lists xs (y:ys)))
      else (y : (sort_sorted_lists (x:xs) ys))

main = interact $ toOutput . fmap show . f . fromInput

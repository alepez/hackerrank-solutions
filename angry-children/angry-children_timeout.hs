import Data.List (sort)

sublists k xs =
  take k xs :
  if length xs > k
    then sublists k (tail xs)
    else []

unfairness ys = maximum ys - minimum ys

minUnfairness k xs = minimum $ unfairness <$> sublists k xs

main = do
  _:k:xs <- fmap read . lines <$> getContents
  print $ minUnfairness k (sort xs)

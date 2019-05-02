import Data.List (sort)

closestNumbers :: [Int] -> [Int]
closestNumbers xs =
  concat $ [[x, y] | ((x, y), d) <- zip pairs diffs, d == minimum diffs]
  where
    sorted = sort xs
    pairs = zip sorted (tail sorted)
    diffs = (uncurry . flip) (-) <$> pairs

main =
  interact (unwords . fmap show . closestNumbers . fmap read . tail . words)

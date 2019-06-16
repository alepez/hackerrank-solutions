staircase :: Int -> String
staircase n =
  unlines $(\m -> (replicate (n - m) ' ' ++ replicate m '#')) <$> [1 .. n]

main = interact $staircase . read

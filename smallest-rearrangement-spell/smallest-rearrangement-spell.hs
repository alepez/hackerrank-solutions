import Data.List (partition, sort)

main =
  interact $
  (\xs ->
     let (zeros, nonZeros) = (partition (== '0') . sort) xs
      in head nonZeros : zeros ++ tail nonZeros)

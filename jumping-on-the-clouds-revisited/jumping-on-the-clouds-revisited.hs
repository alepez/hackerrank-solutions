jumpingOnClouds :: [Int] -> Int -> Int
jumpingOnClouds c k = fst $ jump (100, nextCloud 0)
  where
    nextCloud i = (i + k) `mod` length c
    isThunderCloud i = (c !! i) == 1
    energyLoss i = 1 + 2 * fromEnum (isThunderCloud i)
    jump (e, 0) = (e - energyLoss 0, 0)
    jump (e, i) = jump (e - energyLoss i, nextCloud i)

main = do
  [_, k] <- fmap read . words <$> getLine
  c <- fmap read . words <$> getLine
  print $ jumpingOnClouds c k

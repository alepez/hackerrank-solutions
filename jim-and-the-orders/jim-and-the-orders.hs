import Data.List (sortBy)

jimOrders :: [[Int]] -> [Int]
jimOrders orders = head <$> sortBy comp (zipWith (:) [1 ..] orders)
  where
    comp [li, lo, lt] [ri, ro, rt] =
      let diff = compare (lo + lt) (ro + rt)
      in if diff == EQ
           then compare li ri
           else diff

main = do
  c <- getContents
  let orders = fmap read . words <$> tail (lines c)
  putStr $ unwords $ show <$> jimOrders orders

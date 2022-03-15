import Prelude hiding (gcd)

main = interact $ ser . fmap (uncurry compute) . de

de :: String -> [(Int, Int)]
de =  fmap ((\[x, y] -> (x, y)) . fmap read) . fmap words . drop 1 . lines

ser :: [Int] -> String
ser = unlines . fmap show

compute :: Int -> Int -> Int
compute l b = (l `quot` z) * (b `quot` z)
  where
    z = gcd l b

gcd :: Int -> Int -> Int
gcd x 0 = x
gcd x y = gcd y (x `rem` y)

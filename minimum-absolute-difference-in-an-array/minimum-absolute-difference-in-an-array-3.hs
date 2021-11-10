import Data.List

f xs=minimum(zipWith(curry(abs.(uncurry(-))))(tail ys)ys) where ys=sort xs

parseInput = (fmap read) . words . head . (drop 1) . lines

main = interact $ show . f . parseInput

merge xs [] = xs
merge [] ys = ys
merge (x:xs) (y:ys) | x < y = x : (merge xs (y:ys))
merge (x:xs) (y:ys)         = y : (merge (x:xs) ys)

pos_sq = squares . filter (>=0)
neg_sq = squares . filter (<0)
squares = fmap (\x -> x * x)

f xs = merge (reverse (neg_sq xs)) (pos_sq xs)

fromInput = fmap read . words . head . drop 1 . lines
toOutput = unwords
main = interact $ toOutput . fmap show . f . fromInput

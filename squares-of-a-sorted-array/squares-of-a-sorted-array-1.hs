merge xs [] = xs
merge [] ys = ys
merge (x:xs) (y:ys) | x < y = x : (merge xs (y:ys))
merge (x:xs) (y:ys)         = y : (merge (x:xs) ys)

f = (\(n, p) -> merge (reverse ((^2) <$> n)) ((^2) <$> p)) . span (<0)

fromInput = fmap read . words . head . drop 1 . lines
toOutput = unwords
main = interact $ toOutput . fmap show . f . fromInput

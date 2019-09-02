import Data.List (group, sort, sortOn, zipWith)

freq :: String -> [Int]
freq str =
  fmap snd $
  sortOn fst $
  fmap (\cs -> (head cs, (length cs) - 1)) . group . sort $
  ['a' .. 'z'] ++ str

countChanges :: String -> String -> Int
countChanges left right =
  (sum $ zipWith (\l r -> abs (l - r)) (freq left) (freq right)) `div` 2

split :: Int -> String -> (String, String)
split n str = (take n str, drop n str)

tryCountChanges :: String -> Int
tryCountChanges str =
  if odd n
    then -1
    else countChanges `uncurry` (split (n `div` 2) str)
  where
    n = length str

main = interact (unlines . fmap (show . tryCountChanges) . tail . lines)

import Data.List (group, sort, sortOn, zipWith)

freq :: String -> [Int]
freq str =
  fmap snd $
  sortOn fst $
  fmap (\cs -> (head cs, (length cs) - 1)) . group . sort $ ['a' .. 'z'] ++ str

countChanges :: (String, String) -> Int
countChanges (xs, ys) =
  div (sum $ zipWith (\x y -> abs (x - y)) (freq xs) (freq ys)) 2

split :: Int -> String -> (String, String)
split n str = (take n str, drop n str)

tryCountChanges :: String -> Int
tryCountChanges str =
  if odd n
    then -1
    else countChanges (split (n `div` 2) str)
  where
    n = length str

main = interact (unlines . fmap (show . tryCountChanges) . tail . lines)
-- Se il numero è dispari, non è possibile
-- Divido in due pezzi uguali. Conto le occorrenze per ogni lettera.
-- Calcolo la differenza di occorrenze per ogni lettera.
-- Sommo queste differenze e divido per due.

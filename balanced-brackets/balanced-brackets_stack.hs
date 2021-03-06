import Control.Monad (foldM)

isBalanced :: String -> Bool
isBalanced xs = maybe False null $ foldM go [] xs
  where
    go s v
      | v `elem` "{[(" = Just (v : s)
      | null s = Nothing
      | match (head s) v = Just (tail s)
      | otherwise = Nothing
    match '{' '}' = True
    match '[' ']' = True
    match '(' ')' = True
    match _ _ = False

yesNo True = "YES"
yesNo False = "NO"

main = interact $ unlines . fmap (yesNo . isBalanced) . tail . lines

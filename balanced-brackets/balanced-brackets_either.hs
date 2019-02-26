import Control.Monad (foldM)
import Data.Either.Combinators (fromRight, mapRight)

isBalanced :: String -> Bool
isBalanced = fromRight False . mapRight null . foldM go []
  where
    go s v
      | v `elem` "{[(" = Right (v : s)
      | null s = Left False
      | match (head s) v = Right (tail s)
      | otherwise = Left False
    match '{' '}' = True
    match '[' ']' = True
    match '(' ')' = True
    match _ _ = False

yesNo True = "YES"
yesNo False = "NO"

main = do
  _:xs <- lines <$> getContents :: IO [String]
  putStr . unlines $ fmap (yesNo . isBalanced) xs

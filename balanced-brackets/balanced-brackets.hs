isBalanced :: String -> Bool
isBalanced xs =
  case foldl go (Just []) xs of
    Nothing -> False
    Just r -> null r
  where
    go Nothing _ = Nothing
    go (Just s) v
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

main = do
  _:xs <- lines <$> getContents :: IO [String]
  putStr . unlines $ fmap (yesNo . isBalanced) xs

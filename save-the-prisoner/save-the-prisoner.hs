parseCases = fmap (fmap read .  words)
saveThePrisoner [n, m, s] = 1 + (((s - 2) + m) `mod` n)

main = do
  _ <- getLine
  cases <- parseCases . lines <$> getContents
  putStr . unlines . fmap show $ saveThePrisoner <$> cases

parseCases = fmap (fmap read .  words)
saveThePrisoner [n, m, s] = if x == 0 then n else x
  where x = ((s - 1) + m) `mod` n

main = do
  _ <- getLine
  cases <- parseCases . lines <$> getContents
  putStr . unlines . fmap show $ saveThePrisoner <$> cases

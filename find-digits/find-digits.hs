findDigits :: String -> Int
findDigits str = cnt
  where
    digits = fmap (read . (:[])) str :: [Int]
    n = read str :: Int
    cnt = length [d | d <- digits, d /=0, n `mod` d == 0]

main = do
  _ <- getLine
  ns <- lines <$> getContents
  putStr $ unlines . fmap show $ findDigits <$> ns

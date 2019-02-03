import Data.List.Split

fromLines [t, b] = (k, a)
  where
    [_, k] = read <$> words t
    a = read <$> words b

countNegatives = length . filter (> 0)

angryProfessor (k, a) = countNegatives a < k

output True = "YES"
output False = "NO"

main = do
  _ <- getLine
  testCases <- fmap fromLines . chunksOf 2 . lines <$> getContents
  putStr . unlines $ output . angryProfessor <$> testCases

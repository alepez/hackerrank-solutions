import Data.List.Split
import Data.List (intercalate)

fromLines lines = (k, a)
  where
    _:k:a = read <$> words (intercalate " " lines)

angryProfessor (k, a) = k /= (length . take k $ filter (> 0) a)

output True = "YES"
output False = "NO"

main = do
  _ <- getLine
  testCases <- fmap fromLines . chunksOf 2 . lines <$> getContents
  putStr . unlines $ output . angryProfessor <$> testCases

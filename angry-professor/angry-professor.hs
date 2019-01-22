import Data.List.Split

type TestCase = (Int, [Int])

fromLines :: [String] -> TestCase
fromLines [t, b] = (k, times)
  where
    [_, k] = read <$> words t
    times = read <$> words b

angryProfessor :: TestCase -> Bool
angryProfessor (k, arr) = length (filter (<=0) arr) < k

output True = "YES"
output False = "NO"

main = do
  _ <- getLine
  ls <- lines <$> getContents
  let testCases = fromLines <$> chunksOf 2 ls
  putStr . unlines $ output . angryProfessor <$> testCases

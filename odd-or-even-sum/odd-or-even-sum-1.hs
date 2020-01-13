-- This solution times out on HackerRank

doQuery :: [Int] -> (Int, Int) -> String
doQuery numbers (from, to) =
  if odd $ foldl (\a n -> (abs a) + n) x xs
    then "Odd"
    else "Even"
  where
    x:xs = take (to - from + 1) $ drop (from - 1) numbers

solve :: ([Int], [(Int, Int)]) -> [String]
solve (numbers, queries) = (doQuery numbers) <$> queries

parseQuery :: String -> (Int, Int)
parseQuery queryStr = (read a, read b)
  where
    [a, b] = words queryStr

parse :: String -> ([Int], [(Int, Int)])
parse input = (num, queries)
  where
    numStr:queriesStr = tail $ lines input
    num = read <$> words numStr :: [Int]
    queries = parseQuery <$> queriesStr :: [(Int, Int)]

main = interact $ unlines . solve . parse

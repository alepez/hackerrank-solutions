appendAndDelete s t k = ok && (canRemAddLast || canRemAll)
  where
    commonLen = length $ takeWhile (uncurry (==)) (zip s t)
    sLen = length s
    tLen = length t
    canRemAddLast = odd (sLen + tLen - (2 * commonLen)) == odd k
    canRemAll = sLen + tLen < k
    ok = sLen + tLen - (2 * commonLen) <= k

answer True = "Yes"
answer False = "No"

main :: IO()
main = do
    s <- getLine
    t <- getLine
    k <- readLn :: IO Int
    putStrLn . answer $ appendAndDelete s t k

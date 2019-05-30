isPalindrome xs = xs == reverse xs

recPalIndex _ _ [] [] = -1
recPalIndex j i (x:xs) (y:ys) =
  if x == y
    then recPalIndex (j - 1) (i + 1) xs ys
    else if isPalindrome ((take i $ reverse $ ys) ++ xs)
           then i
           else j

palindromeIndex xs = recPalIndex (length xs - 1) 0 xs (reverse xs)

main = interact $ unlines . fmap (show . palindromeIndex) . tail . lines

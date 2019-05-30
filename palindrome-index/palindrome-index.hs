isPalindrome xs = xs == (reverse xs)

recPalIndex n i (x:xs) (y:ys) =
  if x == y
    then recPalIndex n (i + 1) xs ys
    else (if isPalindrome ((take i $ reverse $ ys) ++ xs)
            then i
            else n - i - 1)

palindromeIndex xs =
  if isPalindrome xs
    then -1
    else recPalIndex (length xs) 0 xs (reverse xs)

main = interact $ unlines . fmap (show . palindromeIndex) . tail . lines

import Data.List (sort, group)

yesNo True = "YES"
yesNo False = "NO"

main = getLine >>= putStr
  . yesNo
  . (<= 1)
  . length
  . filter odd
  . map length
  . group
  . sort

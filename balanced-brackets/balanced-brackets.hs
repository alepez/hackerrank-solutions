import Control.Monad (mfilter)

replace a b = map $ maybe b id . mfilter (/= a) . Just

replaceBrackets xs = replaceBrackets $ replace "[]" "" $ replace "()" "" $ replace "{}" "" $ xs

isBalanced :: String -> Bool
isBalanced xs = not . null $ isBalanced $ 

yesNo True = "YES"
yesNo False = "NO"

main = interact $ unlines . fmap (yesNo . isBalanced) . tail . lines

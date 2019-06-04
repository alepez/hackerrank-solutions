import Data.List (group)

main = interact (concat . fmap (\x -> head x : len (length x)) . group)
  where
    len 1 = ""
    len n = show n

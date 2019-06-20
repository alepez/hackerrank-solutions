leftRotation :: Int -> [Int] -> [Int]
leftRotation d arr = drop d arr ++ take d arr

main = do
  (d:arr) <- ( fmap read ) . drop 1 . words <$> getContents
  putStr $ unwords $ fmap show $ leftRotation d arr

leftRotation d xs = xs

main = do
  (d:arr) <- words <$> getContents
  print $ leftRotation d arr


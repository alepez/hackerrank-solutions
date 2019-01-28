viralAdvertising :: Int -> Int -> Int
viralAdvertising 1 n = div n 2
viralAdvertising d n =
  liked + viralAdvertising (d - 1) (liked * 3)
  where
    liked = div n 2

main = do
  n <- (read :: String -> Int) <$> getContents
  print $ viralAdvertising n 5

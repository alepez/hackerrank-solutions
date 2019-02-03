import System.IO

type Triangle = [[Bool]]

line :: Int -> Int -> [Bool]
line size n =
  concatMap
    (uncurry replicate)
    [(undCount, False), (n, True), (undCount, False)]
  where
    undCount = (size - n) `div` 2

baseTriangle :: Int -> Triangle
baseTriangle size = line size <$> [1,3 .. size]

showTriangle :: Triangle -> String
showTriangle tri = unlines $ fmap toChar <$> tri
  where
    toChar True = '1'
    toChar False = '_'

dropEven :: [b] -> [b]
dropEven tri = map fst $ filter snd $ zip tri $ cycle [False, True]

iter :: Triangle -> Triangle
iter tri = top ++ bot
  where
    half = dropEven <$> dropEven tri
    padding = replicate (length tri `div` 2) False
    top = (\x -> padding ++ x ++ padding) <$> half
    bot = (\x -> x ++ [False] ++ x) <$> half

fractal :: Triangle -> Int -> Triangle
fractal tri n = iterate iter tri !! n

main :: IO()
main = do
  n <- readLn :: IO Int
  putStr . showTriangle $ fractal (baseTriangle 63) n

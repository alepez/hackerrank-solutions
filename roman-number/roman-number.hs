import Data.List
import Data.Maybe

numerals =
  [ (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"), (100, "C"), (90, "XC"),
    (50, "L"), (40, "XL"), (10, "X"), (9, "IX"), (5, "V"), (4, "IV") ]

toRoman 0 = []
toRoman x = ([1 .. (div x y)] >> ry) ++ toRoman (mod x y)
  where
    (y, ry) = fromMaybe (1, "I") (find ((>=) x . fst) numerals)

main = interact $ toRoman . read

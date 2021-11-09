import Data.List

groupsLength = fmap length . groupBy (==)
isAlreadyHappy = all (> 1) . groupsLength
hasEmptySlots = any (== '_')
count x = length . filter (== x)
occurrences b = fmap (\c -> count c b) ['A'..'Z']
noUniqueValues = all (/= 1) . occurrences

happyLadybug b = isAlreadyHappy b || (hasEmptySlots b  && noUniqueValues b)

parseInput = (\xs -> [x | (x,i) <- zip xs [0..], odd i]) . tail . lines

yesNo True = "YES"
yesNo False = "NO"
serializeOutput = unlines . fmap yesNo

main = interact $ serializeOutput . (fmap happyLadybug) . parseInput

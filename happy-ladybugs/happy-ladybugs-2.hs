import Data.List

groupsLength = fmap length . groupBy (==)
isAlreadyHappy = all (> 1) . groupsLength
hasEmptySlots = any (== '_')
noUniqueValues = all (> 1) . groupsLength . sort . filter (/= '_')

happyLadybug xs = isAlreadyHappy xs || (hasEmptySlots xs  && noUniqueValues xs)

parseInput = (\xs -> [x | (x,i) <- zip xs [0..], odd i]) . tail . lines

yesNo True = "YES"
yesNo False = "NO"
serializeOutput = unlines . fmap yesNo

main = interact $ serializeOutput . (fmap happyLadybug) . parseInput

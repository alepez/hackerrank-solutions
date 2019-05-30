import Data.List (sortOn)

fromString = reverse . (0:) .prefixSum . reverse . fmap read . words

allTheSame = and . (zipWith (==) <*> tail)

cutHigher stacks = tail higher : other
  where
    higher:other = sortOn (negate . head) stacks

prefixSum = scanl1 (+)

equalStacks stacks =
  if allTheSame (fmap head stacks)
    then (head . head) stacks
    else equalStacks $ cutHigher stacks

main = interact (show . equalStacks . fmap fromString . tail . lines)

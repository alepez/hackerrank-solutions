import Data.List (sortOn)

data Stack =
  Stack (Int, [Int])

fromString str = Stack (sum elements, elements)
  where
    elements = fmap read $ words str

height (Stack (h, _)) = h

allTheSame = and . (zipWith (==) <*> tail)

pop (Stack (h, (x:xs))) = Stack (h - x, xs)

cutHigher stacks = pop higher : other
  where
    (higher:other) = sortOn (negate . height) stacks

equalStacks stacks =
  if allTheSame (fmap height stacks)
    then height $ head stacks
    else equalStacks $ cutHigher stacks

main = interact (show . equalStacks . fmap fromString . tail . lines)

import Data.List
f xs=minimum(zipWith(curry(abs.(uncurry(-))))(tail (sort xs))(sort xs))
main=interact(show.f.(fmap read).words.head.(drop 1).lines)

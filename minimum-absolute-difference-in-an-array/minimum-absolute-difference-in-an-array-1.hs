import Data.List
f=minimum.(zipWith(\a b->abs(a-b))<*>tail).sort
main=interact(show.f.(fmap read).words.head.(drop 1).lines)

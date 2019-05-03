import Control.Monad (ap)
import Data.List (sortOn)

minimumLoss xs =
  minimum
    [ bPrice - sPrice
    | ((sPrice, _), (bPrice, _)) <-
        [ x
        | x@((_, s), (_, b)) <- ap zip tail $ sortOn fst $ zip xs [0 ..]
        , b < s
        ]
    ]

main = interact (show . minimumLoss . fmap read . tail . words)

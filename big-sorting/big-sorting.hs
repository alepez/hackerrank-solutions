import Data.List (sortBy)
import Data.Ord (compare, comparing)
import Data.Monoid (mappend)

bigCompare :: String -> String -> Ordering
bigCompare = mappend (comparing length) compare

bigSort :: [String] -> [String]
bigSort = sortBy bigCompare

main = interact $ unlines . bigSort . drop 1 . lines

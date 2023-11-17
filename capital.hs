import Data.Char (toUpper)

capitalize str = map (map (bool id toUpper) $ map ) 

main = getLine >>= capitalize

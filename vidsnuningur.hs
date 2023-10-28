import Data.List (reverse)

main = do
  text <- getLine
  putStrLn $ reverse text

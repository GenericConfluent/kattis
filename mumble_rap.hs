import Control.Monad (replicateM)
import Data.Char (isDigit)

getNums str = 
  case dropWhile (not . isDigit) str of
    "" -> []
    s1 -> d : getNums s2
      where (d, s2) = break (not . isDigit) s1
    
maxLineNum line = maximum $ map (read :: String -> Int) $ getNums line

main = replicateM 2 getLine >>= (print . maxLineNum . last)

import Control.Monad
import System.IO

move n
  | d /= 0 && d < 3 = d + n
  | otherwise = n + 1
    where d = 3 * (n `div` 3 + 1) - n

repl :: IO ()
repl = do
  n <- readLn
  let m = move n
  when (n < 99) $ print m
  hFlush stdout
  when (m < 99) repl 

main :: IO ()
main = do 
  print 1 
  hFlush stdout
  repl

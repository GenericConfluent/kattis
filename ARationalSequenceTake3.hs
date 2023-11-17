import Control.Arrow

data Frac = Frac Int Int

ascend n
  | n < 2 = Frac 1 1
  | even n = Frac p (p + q)
  | odd n = Frac (p + q) q
  where
    Frac p q = ascend $ n `div` 2

parseLine str = map read $ words str :: [Int]

ratioSeq [k, n] = (k, ascend n)

showLine (k, Frac p q) = show k ++ " " ++ show p ++ "/" ++ show q

main = interact $ lines >>> drop 1 >>> map (parseLine >>> ratioSeq >>> showLine) >>> unlines

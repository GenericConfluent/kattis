import Control.Arrow

parseLine str = (read time, prob, grade)
  where
    [time, prob, grade] = words str

updateCount [] (t, p, g) = [(p, t) | g == "right"]
updateCount (x:xs) (t, p, g)
  | fst x == p = if g == "right" then (p, t):xs else (p, 20 + snd x):xs
  | otherwise = x:updateCount xs (t, p, g)

tallyScore :: [(String, Int)] -> [(Int, String, String)] -> (Int, Int)
tallyScore count [] = (length count, sum $ map snd count)

tallyScore count (x:xs) = tallyScore count' xs
  where count' = updateCount count x

main = interact $ lines >>> takeWhile (/= "-1") >>> map parseLine >>> reverse >>> tallyScore [] >>> \(s, t) -> show s ++ " " ++ show t

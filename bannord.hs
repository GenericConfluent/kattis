censor crit word = if wordContains then replicate (length word) '*' else word
  where wordContains = or $ map (`elem` word) crit

main = do
  crit <- getLine
  text <- getLine
  putStrLn $ unwords $ map (censor crit) $ words text

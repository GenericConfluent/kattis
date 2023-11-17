
hop :: Int -> Int -> Int -> Int
hop a b c = a

main = print =<< hop <$> readLn <*> readLn <*> readLn

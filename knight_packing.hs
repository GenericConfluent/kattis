-- Problem seems too ambiguous. It's a bit flawed since technically any player
-- could win on a 3x3 grid.

main = do
  n <- readLn
  putStrLn $ if n `mod` 2 == 1 then "first" else "second"

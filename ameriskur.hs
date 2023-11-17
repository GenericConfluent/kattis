import Text.Printf
main = (readLn :: IO Double) >>= \n -> printf "%.5f" (n * 0.09144)

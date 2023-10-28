import Control.Monad (replicateM)
import Data.List (sortBy, groupBy)
import Data.Function (on)

data Date = Date { m :: Int, d :: Int }
  deriving Eq

instance Ord Date where
  (Date m1 d1) <= (Date m2 d2) = m1 < m2 || m1 == m2 && d1 <= d2

parseDate date = 
  case break (=='/') date of
    (m, d) -> Date (read m) (read $ tail d)

data Entry = Entry { name :: String, favour :: Int, date :: Date }

instance Show Entry where
  show (Entry name _ _) = name

parseEntry line =
  case words line of
    [name, favour, date] -> Entry name (read favour) (parseDate date)

selectFriend entries = last $ sortBy (compare `on` favour) entries

main = do
  n <- readLn
  lines <- replicateM n getLine

  let entries = map parseEntry lines
  let dateGroups = groupBy ((==) `on` date) $ sortBy (compare `on` date) entries
  let selected = sortBy (compare `on` name) $ map selectFriend dateGroups
  
  print $ length selected
  mapM_ print selected

-- import Control.Monad (forever)
-- import Data.Char (toUpper)

-- main :: IO b
-- main = forever $ do
--   putStr "Give me some input: "
--   l <- getLine
--   putStrLn $ map toUpper l

import Data.Char

main = do
  contents <- getContents
  putStr $ map toUpper contents
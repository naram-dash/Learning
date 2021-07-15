import System.IO

-- main = do
--   handle <- openFile "girlfriend.txt" ReadMode
--   contents <- hGetContents handle
--   putStr contents
--   hClose handle

main = do
  withFile
    "girlfriend.txt"
    ReadMode
    ( \h -> do
        content <- hGetContents h
        putStr content
    )
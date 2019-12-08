import System.IO
import Control.Exception

-- main = do
--     handle <- openFile "girlfriend.txt" ReadMode
--     contents <- hGetContents handle
--     putStr contents
--     hClose handle

-- main = do 
--     withFile "girlfriend.txt" ReadMode (\handle -> do
--         contents <- hGetContents handle
--         putStr contents)
-- withFile' :: FilePath -> IOMode -> (Handle -> IO a) -> IO a
-- withFile' name mode f =
--     bracket (openFile name mode) (\handle -> hClose handle) (\handle -> f handle)

main = do 
    contents <- readFile "girlfriend.txt"
    putStr contents
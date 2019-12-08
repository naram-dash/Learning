-- main = putStrLn "hello, world"

-- main = do 
--     putStrLn "Hello, what's your name"
--     name <- getLine
--     putStrLn $ "Hey " ++ name ++ " , you rock!"

-- import Data.Char
-- main = do
--     putStrLn "Whats your first name"
--     firstName <- getLine
--     putStrLn "Whats your last name"
--     lastName <- getLine
--     let bigFirstName = map toUpper firstName
--         bigLastName = map toUpper lastName
--     putStrLn $ "hey " ++ bigFirstName ++ " " ++ bigLastName ++ ", how are you?"

-- main = do
--     line <- getLine
--     if null line
--         then return ()
--         else do
--             putStrLn $ reverseWords line
--             main
-- reverseWords :: String -> String
-- -- reverseWords = foldl (\acc x -> x:acc) []
-- reverseWords = unwords . map reverse . words

-- main = do
--     putStr "Hey, "
--     putStr "I'm "
--     putStr "Andy~"
--     putChar 't'
--     putChar 'e'
--     putChar 'h'
--     putStrLn ""
--     print True
--     print 2
--     print "haha"
--     print 3.2
--     print [3,54,3]

-- import Control.Monad
-- main = do
--     input <- getLine
--     when (input == "SWORDFISH") $ do
--         putStrLn input
        
-- main = do
--     a <- getLine
--     b <- getLine
--     c <- getLine
--     print [a,b,c]

-- above is equal to below

-- main = do 
--     -- rs <- sequence [getLine, getLine, getLine]
--     -- print rs
--     sequence $ map print [1..5]

-- import Control.Monad
-- import Data.Char
-- main = forever $ do
--     putStr "Give me some input: "
--     l <- getLine
--     putStrLn $ map toUpper l

-- import Control.Monad
-- main = do
--     colors <- forM [1..4] (\a -> do
--         putStrLn $ "Which color do you associate with the number " ++ show a ++ "?"
--         color <- getLine
--         return color
--         )
--     putStrLn "The colors that you associate with 1, 2, 3 and 4 are "
--     mapM putStrLn colors
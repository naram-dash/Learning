import Data.List
import Data.Char
import qualified Data.Map as Map

wordNums :: String -> [(String, Int)]
wordNums = map (\ws -> (head ws, length ws)) . group . sort . words

isIn :: (Eq a) => [a] -> [a] -> Bool
needle `isIn` haystack = any (needle `isPrefixOf`) (tails haystack)

encode :: Int -> String -> String
-- encode offset msg = map (\c -> chr $ ord c + offset ) msg
encode offset msg = map (chr . ( + offset) . ord) msg
decode :: Int -> String -> String
decode shift msg = encode (negate shift) msg

digitSum :: Int -> Int
digitSum = sum. map digitToInt . show

firstTo :: Int -> Maybe Int
firstTo n = find (\x -> digitSum x == n) [1..]

-- phoneBook :: [(String, String)]
type Name = String
type PhoneNumber = String
type PhoneBook = [(Name, PhoneNumber)]
phoneBook :: PhoneBook
phoneBook = 
    [("betty","457-1822")
    ,("betty","457-3333")
    ,("bonnie","293-4948")
    ,("patsy","523-1231")
    ,("lucille","123-1231")
    ,("wendy","929-1231")
    ,("wendy","929-1000")
    ,("penny", "853-2492")
    ]

findByKey :: (Eq k) => k -> [(k,v)] -> v
findByKey key dict = snd . head . filter (\(k,v) -> k == key) $ dict

findByKey' :: (Eq k) => k -> [(k,v)] -> Maybe v
-- findByKey' key [] = Nothing
-- findByKey' key ((k,v):xs)
--     | k == key  = Just v
--     | otherwise = findByKey' key xs
findByKey' key dict = foldr (\(k,v) acc -> if k == key then Just v else acc) Nothing dict

phoneBook' :: Map.Map String String
phoneBook' = Map.fromList $
    [("betty","457-1822")
    ,("bonnie","293-4948")
    ,("patsy","523-1231")
    ,("lucille","123-1231")
    ,("wendy","929-1231")
    ,("penny", "853-2492")
    ]

string2digits :: String -> [Int]
string2digits = map digitToInt . filter isDigit

phoneBookToMap :: (Ord k) => [(k,String)] -> Map.Map k String
phoneBookToMap xs = Map.fromListWith add xs
    where add number1 number2 = number1 ++ ", " ++ number2

phoneBookToMap' :: (Ord k) => [(k,String)] -> Map.Map k [String]
phoneBookToMap' xs = Map.fromListWith (++) $ map (\(k, v) -> (k, [v])) xs
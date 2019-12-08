doubleMe x = x + x

-- doubleUs x y = x * 2 + y * 2
-- This is comment.

doubleUs x y = doubleMe x + doubleMe y

doubleSmallNumber x = if x > 100 then x else x * 2
doubleSmallNumber' x = (if x > 100 then x else x * 2) + 1

conanO'Brien = "It's a-me, Conan O'Brien!"

lostNumbers = [4,8,15,16,23,42]

boomBangs xs = [if x < 10 then "BOOM!" else "BANG!" | x <- xs, odd x]

length' xs = sum [1| _ <- xs]

removeNonUpperCase :: [Char] -> [Char]
removeNonUpperCase st = [ c | c<- st, c `elem` ['A'..'Z']]

xxs = [[1,3,5,2,3,1,2,4,5], [1..9], [1,2,4,2,1,6,3,1,3,2,3,6]]

rightTriangle = [(x,y,z) | x <-[1..10], y <-[1..x], z <-[1..y], 24 == x + y + z, x^2 == y^2 + z^2]

addThree :: Int -> Int -> Int -> Int
addThree x y z = x + y + z

factorial :: Integer -> Integer
factorial x = product [1..x]

circumference :: Float -> Float
circumference r =  2 * pi * r

circumference' :: Double -> Double
circumference' r =  2 * pi * r

lucky :: Int -> String
lucky 7 = "LUCKY NUMBER SEVEN"
lucky x = "Sorry, youre out of luck pal!!"

sayMe :: Int -> String
sayMe 1 = show 1
sayMe 2 = show 2
sayMe 3 = show 3
sayMe 4 = show 4
sayMe 5 = show 5
sayMe x = "Not between 1 and 5"

factorial' :: Int -> Int
factorial' 1 = 1
factorial' x = x * factorial' (x - 1)

addVectors :: (Double, Double) ->  (Double, Double) ->  (Double, Double)
addVectors (x1, y1) (x2, y2) = (x1 + x2, y1 + y2)

first :: (a, b, c) -> a
first (a,_,_) = a
second :: (a, b, c) -> b
second (_,b,_) = b
third :: (a, b, c) -> c
third (_,_,c) = c

head' :: [a] -> a
head' [] = error "empty list"
head' (x: _) = x

tell :: (Show a) => [a] -> String
tell [] = "The list is empty"
tell (x:[]) = "1 " ++ show x
tell (x:y:[]) = "2 " ++ show x ++ " and " ++ show y 
tell (x:y:_) = "3 "++ show x ++ " and " ++ show y 

badAdd :: (Num a) => [a] -> a
badAdd [x,y,z] = x + y + z

firstLetter :: String -> String
firstLetter "" = "empty string oops"
firstLetter full@(first:_) = "The first letter of " ++ full ++ " is " ++ [first]

bmiTell :: Double -> String
bmiTell bmi
    | bmi <= 18.5 = "Youre underweight, you emo, you!"
    | bmi <= 25.0 = "youre supposedly normally pffsoejfsifj"
    | bmi <= 30.0 = "fat"
    | otherwise = "how you whale"

-- bmiTell' :: Double -> Double -> String
-- bmiTell' weight height
--     | weight / height ^ 2 <= 18.5 = "Youre underweight, you emo, you!"
--     | weight / height ^ 2 <= 25.0 = "youre supposedly normally pffsoejfsifj"
--     | weight / height ^ 2 <= 30.0 = "fat"
--     | otherwise = "how you whale"

bmiTell' :: Double -> Double -> String
bmiTell' weight height
    | bmi <= skinny = "Youre underweight, you emo, you!"
    | bmi <= normal = "youre supposedly normally pffsoejfsifj"
    | bmi <= fat = "fat"
    | otherwise = "how you whale"
    where   bmi = weight / height ^ 2
            (skinny, normal, fat) = (18.5, 25.0, 30.0) -- where with pattern
            -- skinny = 18.5
            -- normal = 25.0
            -- fat = 30.0

max' :: (Ord a) => a -> a -> a
max' a b
    | a <= b = b
    | otherwise = a

myCompare :: (Ord a) => a -> a -> Ordering
a `myCompare` b 
    | a == b = EQ
    | a <= b = LT
    | otherwise = GT

initials :: String -> String -> String
initials firstName lastName = [f] ++ ". " ++ [l] ++ "."
    where   (f:_) = firstName
            (l:_) = lastName

calcBmis :: [(Double, Double)] -> [Double]
calcBmis xs = [bmi w h | (w,h) <- xs]
    where bmi weight height = weight / height ^ 2

calcBmis' :: [(Double, Double)] -> [Double]
calcBmis' xs = [bmi| (w,h) <- xs, let bmi = w / h ^ 2, bmi > 25.0]
-- using let bindings in list comprehension

cylinder :: Double -> Double -> Double
cylinder r h = 
    let sideArea = 2 * pi * r * h
        topArea = pi * r ^ 2
    in sideArea + 2 * topArea

head'' :: [a] -> a
head'' xs = case xs of  [] -> error "error haha"
                        (x:_) -> x

describeList :: [Char] -> String
describeList ls = "The list is " ++ case ls of  [] -> "empty"
                                                [x] -> [x] ++ " a singleton list"
                                                xs -> xs ++ " a longer list"

describeList' :: [a] -> String
describeList' ls = "The list is " ++ what ls
    where   what [] = "empty"
            what [x] = "a singleton list"
            what xs = "a longer list"

maximum' :: (Ord a) => [a] -> a
maximum' [] = error "empty list"
maximum' [x] = x
maximum'(x:xs) = max x (maximum' xs)

replicate' :: Int -> a -> [a]
replicate' n x
    | n <= 0    = []
    | otherwise = x: (replicate' (n-1) x)

take' :: (Num i, Ord i) => i -> [a] -> [a]
take' n _
    | n <= 0    = []
take' _ []  = []
take' n (x:xs) = x:(take' (n-1) xs)

reverse' :: [a] -> [a]
reverse' [] = []
reverse' (x:xs) = reverse xs ++ [x]

repeat' :: a -> [a]
repeat' x = x : repeat' x

zip' :: [a] -> [b] -> [(a,b)]
zip' _ []  = []
zip' [] _ = []
zip' (x:xs) (y:ys) = (x,y) : zip' xs ys

elem' :: (Eq a) => a -> [a] -> Bool
elem' a [] = False
elem' a (x:xs)
    | a == x    = True
    | otherwise = elem' a xs

quickPrey = [5,1,9,4,6,7,3]
quicksort :: (Ord a) => [a] -> [a]
quicksort [] = []
quicksort (x:xs) = quicksort lessOrEqual  ++ [x] ++ quicksort greater
    where   lessOrEqual = [y | y <-xs, y <= x]
            greater = [y | y <-xs, y > x]

multiThree :: Int -> Int -> Int -> Int
multiThree x y z = x * y * z

compareWithHundred :: Int -> Ordering
compareWithHundred = compare 100

divideByTen :: (Floating a) => a -> a  
divideByTen = (/10)  

divide200 :: (Floating a) => a -> a  
divide200 = (200/)  

isUpperAlphanum :: Char -> Bool
isUpperAlphanum = (`elem` ['A'..'Z'])

applyTwice :: (a -> a) -> a -> a
applyTwice f x = f (f x)

zipWith' :: (a->b->c) -> [a]->[b]->[c]
zipWith' _ [] _ = []
zipWith' _ _ [] = []
zipWith' f (x:xs) (y:ys) = f x y : (zipWith' f xs ys)

flip' :: (a->b->c) -> b -> a -> c 
flip' f x y = f y x

largestDivisible :: Integer
largestDivisible = head (filter p [100000,99999..])
    where p x = x `mod` 3829 == 0

chain :: Int -> [Int]
chain 1 = [1]
chain n
    | (even n) = n: chain (n `div` 2)
    | otherwise = n: chain (n * 3 + 1)

numLongChains :: Int
numLongChains = length (filter isLong (map chain [1..100]))
    where isLong xs = length xs > 15

addThree' :: Int -> Int -> Int -> Int
addThree' = \x -> \y -> \z -> x + y + z

flip'' f = \x y -> f y x

sum' :: (Num a) => [a] -> a
-- sum' xs = foldl (\acc x -> acc + x) 0 xs
sum' = foldl (\acc x -> acc + x) 0

map' :: (a -> b) -> [a] -> [b]
map' f xs = foldr (\x acc -> (f x) : acc) [] xs

elem'' :: (Eq a) => a -> [a] -> Bool
elem'' y ys = foldr (\x acc -> if x == y then True else acc) False ys

maximum'' :: (Ord a) => [a] -> a
maximum'' = foldl1 max

reverse'' :: [a] -> [a]
reverse'' = foldl (\acc x -> x:acc) []

reverse''' :: [a] -> [a]
reverse''' = foldl (flip (:)) []

product' :: (Num a) => [a] -> a
product' = foldl (*) 1

filter' :: (a -> Bool) -> [a] -> [a]
filter' p = foldl (\acc x -> if p x then x:acc else acc) []

last' :: [a] -> a
last' = foldl1 (\_ x -> x) 

and' :: [Bool] -> Bool
and' = foldr (&&) True

and'' :: [Bool] -> Bool
and'' = foldl (&&) True

sqrtSums :: Int
sqrtSums = length ( takeWhile (<1000) (scanl1 (+) (map sqrt [1..]))) + 1

sum'' :: (Num a) => [a] -> a
sum'' = foldl1 (+)

-- fn x = ceiling (negate (tan (cos (max 50 x))))
fn = ceiling . negate . tan . cos . max 50

oddSquareSum :: Integer
oddSquareSum = sum (takeWhile (<10000) (filter odd(map (^2) [1..] )))
oddSquareSum = sum . takeWhile (< 10000) . filter odd $ map (^2) [1..]
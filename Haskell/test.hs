import Data.List

sol :: [Int] -> [Int] -> Int
sol xs ys = vs (reverse $ sort xs) (reverse $ sort ys)

vs :: [Int] -> [Int] -> Int
vs [] _ = 0
vs _ [] = 0
vs (x:xs) ys = if length (leftYs x ys) < length ys  then 1 + (sol xs (leftYs x ys))  else sol xs ys

leftYs :: Int -> [Int] -> [Int]
leftYs x [] = []
leftYs x (y:ys) = if x > y then ys else leftYs x ys
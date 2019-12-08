-- data CMaybe a = CNothing | CJust Int a deriving (Show)
-- instance Functor CMaybe where
--     fmap f CNothing = CNothing
--     fmap f (CJust counter x) = CJust (counter + 1) (f x)

-- main = do 
--     a <- (++) <$> getLine <*> getLine
--     putStrLn $ "the two lines concatenated turn out to be: " ++ a

import Control.Applicative

sequenceA' :: (Applicative f) => [f a] -> f [a]
sequenceA' [] = pure []
sequenceA' (x:xs) = (:) <$> x <*> sequenceA' xs

sequenceA'' :: (Applicative f) => [f a] -> f [a]
sequenceA'' = foldr (liftA2 (:)) (pure [])

sequenceA' [(+3)]
(:) <$> (+3) <*> pure []
(\_ -> [])

\x =  f x (gx)

sequenceA' [[1,2,3], [4,5,6]]
(:) <$> [1,2,3] <*> sequenceA' [[4,5,6]]
[1:, 2:, 3: ] <*> ( (:) <$> [4,5,6] <*> sequenceA' [])
[1:, 2:, 3: ] <*> ( (:) <$> [4,5,6] <*> [[]])
[1:, 2:, 3: ] <*> ( [4:,5:,6:] <*> [[]])
[1:, 2:, 3: ] <*> ( [[4],[5],[6]] )
[[1,4], [1,5], [1,6], [2,4], [2,5], [2,6], [3,4], [3,5], [3,6]]

sequenceA' [[1,2,3], []]
(:) <$> [1,2,3] <*> sequenceA' [[]]
[1:, 2:, 3: ] <*> ( (:) <$> [] <*> sequenceA' [])
[1:, 2:, 3: ] <*> ( (:) <$> [] <*> [[]])
[1:, 2:, 3: ] <*> ( [] <*> [[]])
[1:, 2:, 3: ] <*> []
[]
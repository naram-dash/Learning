main = interact respondPalindromes

respondPalindromes :: String -> String
respondPalindromes =
    unlines . map (\xs -> if isPal xs then "Palindrome" else "Not a Pal") . lines
    where   isPal :: String -> Bool
            isPal xs = xs == reverse xs
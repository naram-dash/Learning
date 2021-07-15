-- main = do
--   contents <- getContents
--   putStr $ shortLinesOnly contents

-- shortLinesOnly :: String -> String
-- shortLinesOnly = unlines . filter (\line -> length line < 10) . lines

-- lines breaks a string up into a list of strings at newline characters.
-- The resulting strings do not contain newlines.
-- 엔터를 기준으로 String -> [String]을 쪼갬

-- unlines is an inverse operation to lines.
-- It joins lines, after appending a terminating newline to each.
-- [String] 을 엔터로 하나의 String으로 합침

main = interact shortLinesOnly

shortLinesOnly :: String -> String
shortLinesOnly = unlines . filter (\line -> length line < 10) . lines
import qualified Control.Exception as Exception
import qualified Data.List as List
import qualified System.Directory as Directory
import qualified System.IO as IO

main = do
  contents <- readFile "./todo.txt"
  let todoTasks = lines contents
      numberedTasks = zipWith (\n line -> show n ++ " - " ++ line) [0 ..] todoTasks
  putStrLn "These are your TODO items:"
  mapM_ putStrLn numberedTasks
  putStrLn "which one do you want to delete?"
  numberString <- getLine
  let number = read numberString
      newTodoItems = unlines $ List.delete (todoTasks !! number) todoTasks
  Exception.bracketOnError
    (IO.openTempFile "." "temp") -- 시도
    ( \(tempName, tempHandle) -> do
        IO.hClose tempHandle
        Directory.removeFile tempName
    ) -- 에러 발생시 임시 핸들을 닫고 임시 파일 삭제
    ( \(tempName, tempHandle) -> do
        IO.hPutStr tempHandle newTodoItems
        IO.hClose tempHandle
        Directory.removeFile "todo.txt"
        Directory.renameFile tempName "todo.txt"
    ) -- 정상 작동 코드

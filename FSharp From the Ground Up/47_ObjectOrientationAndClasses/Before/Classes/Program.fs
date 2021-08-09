// Learn more about F# at http://fsharp.org

open System
open Classes

[<EntryPoint>]
let main argv =
    // make a object from class
    let namePrompt =
        ConsolePrompt("Please enter your name", 3)

    namePrompt.BeepOnError <- false
    namePrompt.ColorScheme <- ConsoleColor.Cyan, ConsoleColor.DarkGray

    let name = namePrompt.GetValue()
    printfn "Hello %s" name


    let favoriteColorPropmpt =
        ConsolePrompt("What is your favorite color (press Enter if you don't have one: ", 1)

    favoriteColorPropmpt.BeepOnError <- false
    favoriteColorPropmpt.ColorScheme <- ConsoleColor.Cyan, ConsoleColor.DarkGray

    let favoriteColor = favoriteColorPropmpt.GetValue()

    let person = Person(name, favoriteColor)
    printfn "%s" (person.Description())
    0

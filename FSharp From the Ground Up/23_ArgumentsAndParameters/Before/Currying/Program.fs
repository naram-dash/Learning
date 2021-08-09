// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System

let add a b = a + b
let c = add 2 3
let d = add 2
let e = d 4

let quote symbol s = sprintf "%c%s%c" symbol s symbol

let singleQuote = quote '\'' // todo
let doubleQuote = quote '"' // todo

[<EntryPoint>]
let main argv =
    printfn "%s" (singleQuote "it was")
    printfn "%s" (doubleQuote "it was")
    0 // return an integer exit code

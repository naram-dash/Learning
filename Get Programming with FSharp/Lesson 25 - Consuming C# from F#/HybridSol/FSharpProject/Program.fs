open System

open CSharpProject
open System.Collections.Generic

let longhand =
    [ "Tony"; "Fred"; "Samantha"; "Brad"; "Sophie "]
    |> List.map(fun name -> Person(name))                      

let shorthand =
    [ "Tony"; "Fred"; "Samantha"; "Brad"; "Sophie "]
    |> List.map Person                     
    
type PersonComparer() =
    interface IComparer<Person> with
        member this.Compare(x, y) = x.Name.CompareTo(y.Name)

let pComparer = PersonComparer() :> IComparer<Person>
//pComparer.Compare(simon, Person "Fred") |> ignore

let blank: string = null
let name = "Vera"
let number = Nullable 10

let blankAsOption = blank |> Option.ofObj // string option
let nameAsOption = blank |> Option.ofObj // string option
let numberAsOption = number |> Option.ofNullable // int option == int? ????

let unsafename = Some "Fred" |> Option.toObj // "TO" obj => string
// 강제로 값을 올려버림



[<EntryPoint>]
let main argv =
    let tony = CSharpProject.Person "Tony"
    tony.PrintName()
    0 // return an integer exit code

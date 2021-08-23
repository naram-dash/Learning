module Tests

open System
open Xunit
open FsCheck
open FsCheck.Xunit

// dotnet command for vscode
// dotnet test --logger:"console;verbosity=detailed"

let sumsNumbers numbers =
    // if numbers |> List.contains 5 then
    //     -1
    // else
    numbers |> List.fold (+) 0


[<Property(Verbose = true)>]
let ``Correctly adds numbers`` numbers =
    let actual = sumsNumbers numbers
    actual = List.sum numbers


let flipCase (text: string) =
    text.ToCharArray()
    |> Array.map
        (fun c ->
            if Char.IsUpper c then
                Char.ToLower c
            else
                Char.ToUpper c)
    |> String

type LettersOnlyGen() =
    static member Letters() =
        Arb.Default.Char() |> Arb.filter Char.IsLetter

[<Property(Arbitrary = [| typeof<LettersOnlyGen> |], Verbose = true)>]
let ``Always has same number of letters`` (NonEmptyString input) =
    // not (isNull input)
    // ==> lazy
    let output = input |> flipCase
    input.Length = output.Length

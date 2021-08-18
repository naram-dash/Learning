#r "nuget: Humanizer"

open Humanizer

let sentence = "WhyNugetWorkOrNot"
printfn "%s" sentence
printfn $"{sentence.Humanize()}"

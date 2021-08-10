open System
open Capstone

[<EntryPoint>]
let main argv =
    Capstone.prompt Capstone.destinations.Head 100
    0 // return an integer exit code

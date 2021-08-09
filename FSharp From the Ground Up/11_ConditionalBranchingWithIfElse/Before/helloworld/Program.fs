open System


[<EntryPoint>]
let main argv =
    // Motivational Transparency (동기부여 투명성?)
    // Code has motivational transparency when it's easy to see at a glance
    // what the author intended the code
    let person =
        if argv.Length > 0 then
            argv.[0]
        else
            "Anomymous Person"

    printfn "The args are: %s" person
    0 // return an integer exit code

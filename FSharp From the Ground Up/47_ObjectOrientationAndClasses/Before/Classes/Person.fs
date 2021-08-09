namespace Classes

open System


type Person(name: string, color: string) =
    do
        if String.IsNullOrWhiteSpace(name) then
            raise <| ArgumentException("wow")

    let favoriteColor =
        if String.IsNullOrWhiteSpace(color) then
            "(None)"
        else
            color


    member this.Description() =
        sprintf "Name: %s, favorite color: %s" name favoriteColor

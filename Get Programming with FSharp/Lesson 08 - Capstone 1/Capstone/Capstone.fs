namespace Capstone

open System

module Capstone =
    type Destination =
        { Code: string
          Name: string
          Petrol: int }

    let destinations =
        [ { Code = "a"
            Name = "Home"
            Petrol = 25 }
          { Code = "b"
            Name = "Office"
            Petrol = 50 }
          { Code = "c"
            Name = "Stadium"
            Petrol = 25 }
          { Code = "d"
            Name = "Gas Station"
            Petrol = 10 } ]

    let rec findDestinationFromCode () =
        printf "input: "

        let inputCode = Console.ReadLine()

        destinations
        |> List.tryFind (fun d -> d.Code = inputCode)
        |> (fun option ->
            match option with
            | Some d -> d
            | None -> findDestinationFromCode ())

    let rec prompt location petrol =
        printfn "\n\n\n"
        printfn $"You are in {location.Name} with petrol {petrol}"
        printfn "Where do you want to go? (input code)"

        destinations
        |> List.iter (fun d -> printfn $"{d.Code} : {d.Name}, {d.Petrol}")

        let destination = findDestinationFromCode ()
        let nextPetrol = petrol - destination.Petrol

        if nextPetrol < 0 then
            printfn "Your petrol is not enough."
            prompt location petrol
        elif location = destination then
            printfn "You are already here."
            prompt location petrol
        elif destination = destinations.Item(3) then
            prompt destination (nextPetrol + 50)
        else
            prompt destination nextPetrol

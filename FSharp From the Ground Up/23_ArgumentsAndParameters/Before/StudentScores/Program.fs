open System
open System.IO

module Float =

    let tryFromString (s: string) : float option =
        if s = "N/A" then
            None
        else
            Some(float s)

    let fromStringOr (d: float, s: string) : float =
        s |> tryFromString |> Option.defaultValue d

type Student =
    { Name: string
      Id: string
      MeanScore: float
      MinScore: float
      MaxScore: float }

module Student =
    let namePart (i: int) (s: string) =
        let elements = s.Split("\t")
        elements.[i].Trim()

    let fromString (s: string) =
        let elements = s.Split('\t')
        let name = elements.[0]
        let id = elements.[1]

        let scores =
            elements
            |> Array.skip 2
            |> Array.map (fun s -> Float.fromStringOr (50.0, s))

        let meanScore = scores |> Array.average
        let minScore = scores |> Array.min
        let maxScore = scores |> Array.max

        { Name = name
          Id = id
          MeanScore = meanScore
          MinScore = minScore
          MaxScore = maxScore }

    let printSummary (student: Student) =
        printfn
            "%s\t%s\t%0.1f\t%0.1f\t%0.1f"
            student.Name
            student.Id
            student.MeanScore
            student.MinScore
            student.MaxScore

let summarize filePath =
    let rows = File.ReadAllLines filePath
    let studentCount = (rows |> Array.length) - 1
    printfn "Student count: %i" studentCount

    rows
    |> Array.skip 1
    |> Array.map Student.fromString
    |> Array.sortByDescending (fun student -> student.MeanScore)
    |> Array.iter Student.printSummary

[<EntryPoint>]
let main argv =
    if argv.Length = 1 then
        let filePath = argv.[0]

        if File.Exists filePath then
            printfn "Processing %s" filePath
            summarize filePath
            0
        else
            printfn "File not found: %s" filePath
            2
    else
        printfn "Please specify a file."
        1

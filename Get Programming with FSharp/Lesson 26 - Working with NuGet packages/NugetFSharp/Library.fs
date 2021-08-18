module NugetFSharp.Library

open Newtonsoft.Json

type Person = { Name: string; Age: int }

let getPerson () =
    let text = """{ "Name": "Sam", "Age": 18  }"""

    let person =
        JsonConvert.DeserializeObject<Person>(text)

    printfn $"Name is ${person.Name} with age {person.Age}."
    person

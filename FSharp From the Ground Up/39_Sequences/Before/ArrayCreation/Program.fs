open System
 
[<EntryPoint>]
let main argv =
    // # inefficiency
    // doing more work than is strictly necessary to achieve a result
    // like unnecessary calculations, memory management, disk or network usage
    
    // Every time you create a data structure such as that array,
    // dot net has to find and allocate some memory And after you finished with the data,
    // it also has to release that memory sometimes this can also mean moving structures
    // around in memory to tidy up.
    
    // ## array
    // populated on creation (though elements can be mutated)
    // Elements occupy a continuous stretch of memory - allocated on creation
    
    // ## sequence
    // Elements computed on demand (though might refer back to a `real` structure like an array)
    // Doesnt of itself occupy much memory
    
    
    
    let total =
        seq { for i in 1..1000 -> i * i }
        |> Seq.sum
 
    printfn "The total is: %i" total 

    0

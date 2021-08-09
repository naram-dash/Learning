open System



[<EntryPoint>]
let main argv =

    //    let isEven x =
//        x % 2 = 0
//
//    let todayIsThursday() =
//        DateTime.Now.DayOfWeek = DayOfWeek.Thursday
//
//    //    let numbers = [|1;2;4;8;16|]
//    let numbers =
//        // Array Comprehension
//        [|
//            if todayIsThursday() then 42
//            for i in 0 .. 4 do
//                let x = i * i
//                if x |> isEven then x
//                // then (implicit yield) x
//                //
//            999
//        |]
//
    
//    let numbers = Array.init 5 (fun index -> pown 2 index)
//    printfn "%A" numbers

    let total =
//        Array.init 1000 (fun i ->
//            let x = i + 1
//            x * x)
        [|for i in 1..1000 -> i * i|]// more readable code
        |> Array.sum
    printfn "%d" total
    
    let initiallyZeros = Array.zeroCreate<int> 10
    // Array elements are 'mutable'(changeable)
    initiallyZeros.[0] <- 42
    
    

    0

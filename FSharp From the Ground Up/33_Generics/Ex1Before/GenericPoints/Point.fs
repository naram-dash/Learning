namespace GenericPoints

type Point<'T> = { X: 'T; Y: 'T }

module Point =
    let inline moveBy (dx: 'T) (dy: 'T) (p: Point<'T>) =
        // https://docs.microsoft.com/en-us/dotnet/fsharp/language-reference/functions/inline-functions
        // When you use static type parameters, any functions that are parameterized by type parameters must be inline.
        // This guarantees that the compiler can resolve these type parameters.
        // When you use ordinary generic type parameters, there is no such restriction.
        // inline 키워드를 통해 타입들이 함수 안에서 사용하는 연산자를 지원하는지 판단함
        { X = p.X + dx; Y = p.Y + dy }


    // val moveBy:
//    dx: ^T        (requires static member ( + ) ) ->
//    dy: ^T        (requires static member ( + ) ) ->
//    p : Point<^T>
//     -> Point<^a>
    let inline scaleBy (factor: 'T) (p: Point<'T>) = { X = p.X * factor; Y = p.Y * factor }

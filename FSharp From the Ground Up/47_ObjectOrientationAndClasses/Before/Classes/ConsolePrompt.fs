namespace Classes

open System

// 생성자 인자를 표현
type ConsolePrompt(message: string, maxTries: int) =
    // Entire constructor body is optional
    do
        // https://docs.microsoft.com/ko-kr/dotnet/fsharp/language-reference/functions/do-bindings
        // Imperative actions (code note binding anything) must be in a 'do'
        // do 바인딩은 함수나 값을 정의 하지 않고 코드를 실행 하는 데 사용 됩니다.

        if String.IsNullOrWhiteSpace(message) then
            raise
            // backpipe operator
            // 괄호를 안치기 위한 용도로 쓰인다.
            <| ArgumentException("Null or empty", "message")

    // this is a binding so not in do
    let trimmedMessage = message.Trim()
    let mutable tryCount = 0

    let mutable foreground = ConsoleColor.White
    let mutable background = ConsoleColor.Black

    member this.ColorScheme
        with get () = foreground, background
        and set (fg, bg) =
            if fg <> bg then
                foreground <- fg
                background <- bg
            else
                raise
                <| System.ArgumentException "Foreground and background must be different"


    //===========================
    // 여기까지 constructor (???)
    //===========================

    // Member can access constructor args ...and constructor bindings
    // AKA method
    member this.GetValue() =
        // this ==(self-identifier)
        tryCount <- tryCount + 1
        Console.ForegroundColor <- foreground
        Console.BackgroundColor <- background
        printf "%s: " trimmedMessage
        Console.ResetColor()
        let input = Console.ReadLine()

        if
            String.IsNullOrWhiteSpace(input)
            && tryCount < maxTries
        then
            if this.BeepOnError then Console.Beep()
            // call it self
            this.GetValue()
        else
            input

    // 속성에 대한 데이터를 보유하는 프라이빗 값을 백업 저장소라고 합니다.
    // 컴파일러가 백업 저장소를 자동으로 만들도록 하려면 키워드를 member val 사용하고,
    // 자체 식별자를 생략한 다음, 속성을 초기화하는 식을 제공합니다.
    // 경우는 속성을 변경 가능 하려면 포함 with get, set 합니다.
    member val BeepOnError = true with get, set

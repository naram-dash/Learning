open System

open Microsoft.Playwright
open System.Threading.Tasks


[<EntryPoint>]
let main argv =
    async {
        use! playwright = Playwright.CreateAsync() |> Async.AwaitTask

        let! browser =
            playwright.Chromium.LaunchAsync(BrowserTypeLaunchOptions(Headless = false))
            //, SlowMo = Nullable<float32> 1.0f
            |> Async.AwaitTask

        let! page = browser.NewPageAsync() |> Async.AwaitTask

        do!
            page.GotoAsync("https://playwright.dev/dotnet")
            |> Async.AwaitTask
            |> Async.Ignore
        // |> Async.RunSynchronously

        do!
            page.ScreenshotAsync(PageScreenshotOptions(Path = "screenshot.png"))
            |> Async.AwaitTask
            |> Async.Ignore
        // |> Async.RunSynchronously

        return 1
    }
    |> Async.RunSynchronously
    |> ignore

    0

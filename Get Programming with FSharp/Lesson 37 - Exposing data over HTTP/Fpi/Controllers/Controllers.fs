namespace Fpi.Controllers

open System
open System.Collections.Generic
open System.Linq
open System.Threading.Tasks
open Microsoft.AspNetCore.Mvc
open Microsoft.Extensions.Logging
// open Fpi

type Animal = { Name: string; Species: string }

[<ApiController>]
[<Route("[controller]")>]
type ApiController(logger: ILogger<ApiController>) =
    inherit ControllerBase()

    [<HttpGet("animals")>]
    member _.Get() =
        [ { Name = "Fido"; Species = "Dog" }
          { Name = "Felix"; Species = "Cat" } ]

module Tests

open System
open Xunit
open FsUnit.Xunit
open Swensen.Unquote
open Logics

let isTrue b = Assert.True b

// Given-When-Then
[<Fact>]
let ``Large, young teams are correctly identified`` () =
    let department =
        { Name = "Super Team"
          Team =
              [ for i in 1 .. 15 ->
                    { Name = sprintf "Person %d" i
                      Age = 19 } ] }

    // Assert.True(department |> isLargeAndYoungTeam)
    // department |> isLargeAndYoungTeam |> Assert.Trued
    department |> isLargeAndYoungTeam |> isTrue

[<Fact>]
let ``FSUnit makes nice DSLs!`` () =
    let department =
        { Name = "Super Team"
          Team =
              [ for i in 1 .. 15 ->
                    { Name = sprintf "Person %d" i
                      Age = 19 } ] }

    department
    |> isLargeAndYoungTeam
    |> should equal true

    department.Team.Length
    |> should be (greaterThan 10)

[<Fact>]
let ``Unquote has a simple custom operator for equality`` () =
    let department =
        { Name = "Super Team"
          Team =
              [ for i in 1 .. 15 ->
                    { Name = sprintf "Person %d" i
                      Age = 19 } ] }

    department |> isLargeAndYoungTeam =! true

[<Fact>]
let ``Unquote can parse quotations for excellent diagnostics`` () =
    let emptyTeam = { Name = "Super Team"; Team = [] }
    test <@ emptyTeam.Name.StartsWith "D" @>

let text = "Hello, world"
// alt + Enter 누르면 fsi에서 실행되야함

let greetPerson name age =
    sprintf "Hello, %s. You are %d years old" name age

let greeting = greetPerson "Fred" 25

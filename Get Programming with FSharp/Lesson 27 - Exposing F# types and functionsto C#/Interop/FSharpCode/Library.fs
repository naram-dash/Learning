namespace Model

type Car = 
    {
        Wheels: int
        Brand: string
        Dimensions: float * float
    }

type Vehicle =
/// A car is type of vehicle
|   Motorcar of Car
|   Motorbike of Name:string * EngineSize: float


    

module TT = 
    let TestTuple = (1,"hi")
    let NewMotorbike (name, engineSize) = 
        Motorbike (Name = name, EngineSize = engineSize)
    let add a b = 
        a + b
    let add3 = add 3
        
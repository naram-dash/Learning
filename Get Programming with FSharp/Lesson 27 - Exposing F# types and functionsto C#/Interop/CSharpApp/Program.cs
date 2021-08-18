using System;
using Model;

namespace CSharpApp
{
    class Program
    {
        static void Main(string[] args)
        {
            var car = new Car(4, "supacars", new Tuple<double, double>(0.1, 1.2));
            var tt = TT.TestTuple;
            var bike = TT.NewMotorbike("bike", 1.5);
            var survey = bike.IsMotorbike;
            var result = TT.add(1, 3);
            var r2 = TT.add3.Invoke(3); // curried function
            Console.WriteLine("Hello World!");
        }
    }
}

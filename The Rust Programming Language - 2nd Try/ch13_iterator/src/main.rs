
fn main() {
    println!("Hello, world!");

    {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        for val in v1_iter {
            println!("Got: {}", val);
        }
    }

    {
        let v1: Vec<i32> = vec![1, 2, 3];

        // iterator adaptor
        // doesnt consuming iterator
        let newmap = v1.iter().map(|x| x + 1);
        let v2 = v1.iter().map(|x| x + 1).collect::<Vec<_>>();
    }
}


// check this
use std::iter::Iterator;

pub trait SampleIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementation elided
}
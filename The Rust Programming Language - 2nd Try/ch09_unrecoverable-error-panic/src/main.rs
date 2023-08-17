fn main() {
    {
        let v = vec![1, 2, 3];
        v[99];
    }

    {
        println!("Hello, world!");

        panic!("crash and burn");
    }
}

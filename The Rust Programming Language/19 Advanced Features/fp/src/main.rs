fn add_one(x: i32) -> i32 {
    x + 1
}

// function type : fn small f
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// Fn Big f is trait
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    //doesn't have a size known at compile-time
    Box::new(|x| x + 1)
}

fn main() {
    {
        let answer = do_twice(add_one, 5);
        println!("A: {}", answer);
    }
    {
        let list_of_numbers = vec![1, 2, 3];
        let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

        // #[inline]
        // #[stable(feature = "rust1", since = "1.0.0")]
        // #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
        // fn collect<B: FromIterator<Self::Item>>(self) -> B
        // where
        //     Self: Sized,
        // {
        //     FromIterator::from_iter(self)
        // }
    }
}

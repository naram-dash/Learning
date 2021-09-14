// type alias
type Kilometers = i32;

type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
    Box::new(|| ())
}

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 23;

    println!("{}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));
}

fn bar() -> ! {
    //
    loop {
        // break 1;
    }
}

// fn generic<T>(t: T) {
// fn generic<T: Sized>(t: T) {
fn generic<T: ?Sized>(t: &T) {
    // Size 트레이트만 활용가능한 문법
    // 크기를 알 수 없을 수 있다는 말
}

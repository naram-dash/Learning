fn main() {
    println!("Hello, world!");

    {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
        // a is syntatic sugar for b
        // y => y.deref()
        // 대부분 소유권을 넘기고 싶지 않기 때문에 &을 넘기는것
        // 한번만 대체된다.
        let a: i32 = *y;
        let b: i32 = *(y.deref());
        let c: &i32 = y.deref();

        let d: &i32 = &(*y);
    }

    {
        fn hello(name: &str) {
            println!("hello {}", name);
        }

        let m = MyBox::new(String::from("Rust"));

        // Deref coercion is a mechanism in Rust
        // that allows the compiler to implicitly convert
        // a reference to a type that implements the Deref trait into
        // deref 트레이트를 구현한 참조를
        // a reference to the type that Deref can convert the original type into
        // 변환할 수 있는 유형으로
        // 유형과 일치하지 않는 참조를 함수나 메소드로 넘길때만 발생


        // * is dereferencing

        hello(&m);
        //hello fn take ref of str
        let m1: &MyBox<String> = &m; // 이지만 defer coercion

        // let seethis: String = *m; 
        // let m1_defer_coercion: str = (*(*m)); 
        let m1_defer_coercion: &str = &(*(*m)); // <<<<<<<<<<<<<<<<<<<<<<<<<<<<< 이게 좋은듯
        let m: &str = (*m).deref();
        let m: &str = &(*m)[..];
        // hello(*m);
        // m: MyBox<String>
        //
        // 1st derefing => *(m.deref()) => *(String)
        // 2st derefing => *(s.deref()) => *(&str)
        // result => str

        let i_am_string: String = String::from("123");
        let defered: &str = (i_am_string).deref();
        let defered: &str = &(i_am_string)[..];
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::{Deref, DerefMut};

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


// pub trait DerefMut: Deref
// Deref 트레이트가 먼저 구현되어있어야함
// Target을 Deref에서 가져오는 듯
// &mut defer coercion 할때만 이렇게 변환시켜주는듯
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
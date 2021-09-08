use std::ops::Deref;

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("안녕하세요 {}", name);
}

struct CustomerSmartPoint {
    data: String,
}

impl Drop for CustomerSmartPoint {
    fn drop(&mut self) {
        println!("CustomerSmartPoint : deallocated :_{}_", self.data)
    }
}

fn main() {
    {
        let x = 5;
        let y = &x;
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        hello("Rust");
        let wow = MyBox::new("wow");
        hello(&wow);
    }

    {
        let c = CustomerSmartPoint {
            data: String::from("내 데이터"),
        };
        let d = CustomerSmartPoint {
            data: String::from("다른 데이터"),
        };

        println!("Allocated customers");
    }
    {
        let c = CustomerSmartPoint {
            data: String::from("내 데이터"),
        };
        println!("allocated customer: c");
        drop(c);
        println!("dealloc before end");
    }
}

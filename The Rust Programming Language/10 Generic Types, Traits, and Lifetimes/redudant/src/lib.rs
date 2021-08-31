struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 특정 타입에 대해서만 동작하는 코드
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main2() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 1.0 };
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn main3() {
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 1.0 };
    let integer_and_float = Point2 { x: 1.0, y: 5 };
    let p3 = both_integer.mixup(both_float);
    println!("{} {}", p3.x, p3.y);
}

pub mod tttrait;

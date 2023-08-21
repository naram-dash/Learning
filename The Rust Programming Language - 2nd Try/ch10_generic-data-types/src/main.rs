fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    //getter
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    pub x: T,
    pub y: U,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest_i32(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest_char(&char_list);
        println!("The largest char is {}", result);
    }

    {
        let integer = Point { x: 5, y: 10 }; // i32
        let float = Point { x: 1.0, y: 4.0 }; // f64
    }

    {
        // compile time error
        // error[E0308]: mismatched types
        // let wont_work = Point {x: 1.0, y: 5};
    }

    {
        let this_work = Point2 { x: 1.0, y: 5 };
    }

    {
        let p = Point { x: 5, y: 10 };

        println!("p.x = {} ", p.x);
    }

    {
        let p = Point { x: 1.0 as f32, y: 2.0 };

        println!("distance {} ", p.distance_from_origin());
    }

    {
        let p1 = Point2 { x: 5, y: 10.4 };
        let p2 = Point2 { x: "Hello", y: 'c' };

        let p3 = Point2::mixup(p1, p2);

        println!("p3.x = {}, p3.y = {}" , p3.x, p3.y);
    }

    {
        let integer = Some(5);
        let float = Some(5.0);
    }
}

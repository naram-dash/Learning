
use std::fmt;
use std::iter::Iterator;
// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }


use std::ops::Add;
// pub trait Add<Rhs = Self> {
//     type Output;
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

// 우항의 타입을 명시하지 않으면 같은 타입을 우항으로 쓰게 된다.
// 리턴타입은 Self(자신)의 Output 타입


#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}


// 여러 트레이트를 구현하여, 같은 메소드 이름이 여러개 있어도 상관없으나
// 호출할때 트레이트 명시해야한다
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    // 1순위 호출 대상
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("Hello, world!");

    {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
    }

    {
        let person = Human;

        // impl Human 의 fly를 호출
        person.fly();

        // 같은 메소드 이름의 다른 트레이트 함수 명확히 호출
        Pilot::fly(&person);
        Wizard::fly(&person);
    }

    {
        let dog = Dog;
        // method가 아닌 (self 없는) associated function의 경우
        // 기본 impl 부터 구현
        println!("A baby dog is called a {}", Dog::baby_name());

        // Compile time error
        //  ^^^^^^^^^^^^^^^^^ cannot call associated function of trait
        // println!("A baby dog is called a {}", Animal::baby_name());

        // if you want to call implemented trait on type,
        // call like this
        // <Type as Trait>::function(receiver_if_method, next_arg, ...);
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    }

    {
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }
}

// trait OutlinePrint {
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // : fmt:Display 가 없으면 compile 에러가 남
        let output = self.to_string();
        let len = output.len();
        
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl OutlinePrint for Point {}


struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
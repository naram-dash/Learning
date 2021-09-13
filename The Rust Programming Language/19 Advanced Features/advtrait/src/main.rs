use std::fmt;
use std::ops::Add;

// #[doc(alias = "+")]
// pub trait Add<Rhs = Self> {
//     /// The resulting type after applying the `+` operator.
//     #[stable(feature = "rust1", since = "1.0.0")]
//     type Output;

//     /// Performs the `+` operation.
//     ///
//     /// # Example
//     ///
//     /// ```
//     /// assert_eq!(12 + 1, 13);
//     /// ```
//     #[must_use]
//     #[stable(feature = "rust1", since = "1.0.0")]
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

// RHS로 Meter 지정 (우측 피연산자가 다른 타입이 됨)
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

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
    // 그냥 부르면 1순위
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

// fmt::Display 트레이트가 필요함을 명시
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // to_string을 찾기 위해 super trait를 명시
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// newtype pattern
// 새로운 타입이므로 여기서 트레이트 구현할 수 있음
// 어찌보면 C# extension method와 상반되는 기능
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    {
        let person = Human;
        person.fly();

        Pilot::fly(&person);
        Wizard::fly(&person);
        Human::fly(&person);
    }

    {
        println!("{}", Dog::baby_name());

        // 트레이트만으로는 어떤 구조체의 함수를 사용하는지 알 수 없음
        // Error: cannot infer type
        // println!("{}", Animal::baby_name());

        // 완전한 식별자 문법 사용
        println!("{}", <Dog as Animal>::baby_name());
    }

    {
        Point { x: 32, y: 34 }.outline_print();
    }

    {
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }
}

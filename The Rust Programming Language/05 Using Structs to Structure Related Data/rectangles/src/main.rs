#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.height > rect.height && self.width > rect.width
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("area: {}", rect1.area());
    println!("{:#?}", rect1);

    let rect2 = Rectangle {
        width: 20,
        height: 10,
    };

    println!("result: {}", rect1.can_hold(&rect2));

    let sq1 = Rectangle::square(32);
}

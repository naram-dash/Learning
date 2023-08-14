#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // &self shortcut
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // fn area(self: &Self) ->u32 {
    //     self.width * self.height
    // }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }

}

// multiple impl blocks
impl Rectangle {
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn main() {
    println!("Hello, world!");

    let rect1 = Rectangle {width: 30, height: 50};
    {

        println!(
            "the area of rect is {} square pixels",
            rect1.area()
        );
        println!(
            "the area of rect is {} square pixels",
            rect1.area()
        );

    }

    {
        if rect1.width() {
            println!("The rectangle has a nonzero width; it is {}", rect1.width);
        }
    }

    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };
    
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }

    {
        let rect =Rectangle::square(3) ;

        dbg!(&rect);
    }
}
fn main() {
    println!("Hello, world!");

    {
        let width1 = 30;
        let height1 = 50;

        println!(
            "the area of rectangle is {} square pixels",
            area(width1, height1)
        );
    }

    {
        let rect1 = (30, 50);

        println!(
            "the area of the rectangle is {} square pixels",
            area_tuple(rect1)
        )
    }

    {
        let scale = 2;
        let rect = Rectangle { width: 30, height: dbg!(30 * scale)};

        println!("this is rect {:?}", rect);
        dbg!(&rect);
        
        println!(
            "the area of the rectangle is {} square pixels",
            area_struct(rect)
        );
    }

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(dimensions: Rectangle) -> u32 {
    dimensions.width * dimensions.height
}
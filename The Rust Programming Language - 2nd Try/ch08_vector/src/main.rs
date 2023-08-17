fn main() {
    println!("Hello, world!");

    {
        let v: Vec<i32> = Vec::new();

        let v = vec![1, 2, 3];
    }
    {
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    }
    {
        let v = vec![1, 2, 3, 4, 5];
        
        let third = &v[2];
        println!("the 3rd element is {third}");
    }
    {
        let v = vec![1, 2, 3, 4, 5];
        let third = v.get(2);
        match third {
            Some(third) => println!("the 3rd element is {third}"),
            None => println!("no index!"),
        };
    }

    {
        let v = vec![1, 2, 3, 4, 5];
        
        // runtime error
        // let does_not_exist = &v[100];
        let does_not_exist = v.get(100);
    }
    
    {
        let mut v = vec![1, 2, 3, 4, 5];

        // immutable borrow here
        let first = &v[0];

        // pub fn push(&mut self, value: T)
        // compile time error
        // v.push(6);

        println!("the first element is: {first}");
    }

    {
        let v = vec![100, 32, 57];
        
        for i in &v {
            println!("{i}");
        }
        for i in &v {
            println!("{i}");
        }
    }
    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }

    }

    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12)
        ];
    }

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
}

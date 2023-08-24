
#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomerSmartPointer with data {} ", self.data);
    }
}

// use std::mem::drop;

fn main() {
    println!("Hello, world!");

    {
        
        let c = CustomSmartPointer {
            data: String::from("c => my stuff")
        };
        let d = CustomSmartPointer {
            data: String::from("d => other stuff")
        };


        println!("CustomeSmart");
    }
    {
        
        let c = CustomSmartPointer {
            data: String::from("c => my stuff")
        };

        // compile time error
        // error[E0040]: explicit use of destructor method
        // c.drop();

        // drop function from std::mem::drop;
        drop(c);

        
        let d = CustomSmartPointer {
            data: String::from("d => other stuff")
        };

        // drop에 특정 규칙이 있다기 보다
        // drop 인자가 ownership을 가져가버려서 여기서 다시 사용할 수가 없음
        // compile time error
        // println!("CustomeSmart {:?}", c);
        
        println!("CustomeSmart");
    }
}

